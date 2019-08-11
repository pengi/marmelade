use r68k_emu::ram::{
    AddressBus,
    AddressSpace
};
use crate::phy::prefix::Prefix;
use super::Toolbox;
use std::rc::Weak;
use crate::types::{
    PString,
    OSType
};
use crate::serialization::{SerialReadStorage, SerialRead};

const SEGMENT_MAX_SIZE : u32 = 0x8000;

#[derive(SerialRead, Default)]
struct JumpTableHeader {
    _above_a5: u32,
    _below_a5: u32,
    _length: u32,
    offset_a5: u32
}

#[derive(Default)]
pub struct SegmentLoader {
    address_base: u32,
    address_prefix: u32,
    toolbox: Weak<Toolbox>,
    jump_table_header: JumpTableHeader,
    data: Vec<(i16, Vec<u8>)>
}


impl SegmentLoader {
    pub fn new() -> SegmentLoader {
        Default::default()
    }

    pub fn set_prefix(&mut self, address_base: u32, address_prefix: u32) -> Prefix {
        self.address_base = address_base;
        self.address_prefix = address_prefix;
        Prefix::new(self.address_base, self.address_prefix)
    }

    pub fn get_a5(&self) -> u32 {
        self.address_base - self.jump_table_header.offset_a5
    }

    pub fn set_toolbox(&mut self, toolbox: Weak<Toolbox>) {
        self.toolbox = toolbox;
        // New toolbox, therefore reload data
        self.data = vec![];
        self.load(0); // Reload jump table
    }

    fn update_jump_table(&mut self, id: i16, address: u32) {
        // Only update if first element is the jump table
        if let Some((0, jt)) = self.data.get_mut(0) {
            for i in (16..jt.len()).step_by(8) {
                if let Some(seg) = jt.get_mut(i..i+8) {
                    // If segment is a load-segment trap instruction
                    if seg[2] == 0x3f && seg[3] == 0x3c && seg[6] == 0xa9 && seg[7] == 0xf0 {
                        let offset = (seg[0] as u32) << 8 | (seg[1] as u32);
                        let cur_id = ((seg[4] as u32) << 8 | (seg[5] as u32)) as i16;
                        if cur_id == id {
                            // The offset is not including the segment header
                            let new_address = offset + address + 4;
                            seg[0] = seg[4]; // Move resource id to first
                            seg[1] = seg[5];
                            seg[2] = 0x4e; // Jump to immediate long address
                            seg[3] = 0xf9;
                            seg[4] = ((new_address >> 24) & 0xff) as u8;
                            seg[5] = ((new_address >> 16) & 0xff) as u8;
                            seg[6] = ((new_address >> 8) & 0xff) as u8;
                            seg[7] = ((new_address >> 0) & 0xff) as u8;
                        }
                    }
                }
            }
        }
    }

    pub fn load(&mut self, id: i16) -> Option<u32> {
        if let Some(toolbox) = self.toolbox.upgrade() {
            let name = toolbox.rsrc.name(OSType::from(b"CODE"), id).ok()?.unwrap_or(PString::from("-"));

            // See if already loaded
            for (idx, (cur_id, _)) in self.data.iter().enumerate() {
                if *cur_id == id {
                    let address = idx as u32 * SEGMENT_MAX_SIZE + self.address_base;
                    println!("Segment loader: already loaded: {} {} @{:08x}", id, name, address);
                    return Some(address);
                } 
            }

            let idx = self.data.len();
            let address = idx as u32 * SEGMENT_MAX_SIZE + self.address_base;

            let mut data = toolbox.rsrc.open(OSType::from(b"CODE"), id).ok()?;

            // Special case for jump table, load the globals
            if id == 0 {
                self.jump_table_header = JumpTableHeader::read(&mut data).ok()?;
            }


            let data = data.to_vec();
            self.data.push((id, data));

            println!("Segment loader: loading: {} {} @{:08x}", id, name, address);

            self.update_jump_table(id, address);

            Some(address)
        } else {
            println!("Can't load segment {}", id);
            None
        }
    }
}

impl AddressBus for SegmentLoader {
    fn read_byte(&self, _address_space: AddressSpace, address: u32) -> u32 {
        let segment_idx = address / SEGMENT_MAX_SIZE;
        let segment_offset = address % SEGMENT_MAX_SIZE;
        if let Some((_, data)) = self.data.get(segment_idx as usize) {
            // 4 bytes header on segment
            *data.get(segment_offset as usize).unwrap_or(&0xff) as u32
        } else {
            0xff
        }
    }
    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
        (self.read_byte(address_space, address) << 8) | self.read_byte(address_space, address+1)
    }
    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
        (self.read_word(address_space, address) << 16) | self.read_word(address_space, address+2)
    }

    fn write_byte(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        println!("Can't write to segment {:08x} - {:02x}", address, value);
    }
    fn write_word(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        println!("Can't write to segment {:08x} - {:04x}", address, value);
    }
    fn write_long(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        println!("Can't write to segment {:08x} - {:08x}", address, value);
    }
}