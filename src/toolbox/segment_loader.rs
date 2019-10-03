use crate::cpu::{
    CPU,
    CPUPeripheral,
    CPUCore,
    AddressRange,
    Stackable
};
use crate::types::{
    PString,
    OSType
};
use crate::filesys::rsrc::Rsrc;
use std::rc::Rc;
use crate::serialization::{SerialReadStorage, SerialRead};

const SEGMENT_MAX_SIZE : u32 = 0x8000;

#[derive(SerialRead, Default, Clone)]
struct JumpTableHeader {
    _above_a5: u32,
    _below_a5: u32,
    _length: u32,
    offset_a5: u32
}

#[derive(SerialRead, Default)]
struct SegmentHeader {
    _offset: u16,
    _count: u16
}

enum Header {
    JumpTable(JumpTableHeader),
    Segment(SegmentHeader)
}

impl Header {
    fn read(rdr: &mut SerialReadStorage, code_id: i16) -> std::io::Result<Header> {
        if code_id == 0 {
            Ok(Header::JumpTable(SerialRead::read(rdr)?))
        } else {
            Ok(Header::Segment(SerialRead::read(rdr)?))
        }
    }
}

pub struct SegmentLoader {
    address_range: AddressRange,
    rsrc: Rc<Rsrc>,
    jump_table_header: JumpTableHeader,
    data: Vec<(i16, Header, Vec<u8>)>
}


impl SegmentLoader {
    pub fn new(address_range: AddressRange, rsrc: &Rc<Rsrc>) -> SegmentLoader {
        let mut sl = SegmentLoader {
            address_range,
            rsrc: Rc::clone(rsrc),
            jump_table_header: Default::default(),
            data: vec![]
        };
        if let Some(_) = sl.load(0) {
            if let Header::JumpTable(tblheader) = &sl.data[0].1 {
                sl.jump_table_header = tblheader.clone();
            }
        }
        sl
    }

    pub fn get_a5(&self) -> u32 {
        self.address_range.start() - self.jump_table_header.offset_a5
    }

    pub fn get_start(&self) -> u32 {
        self.address_range.start() + 2
    }

    fn update_jump_table(&mut self, id: i16, address: u32) {
        // Only update if first element is the jump table
        if let Some((0, _, jt)) = self.data.get_mut(0) {
            for i in (0..jt.len()).step_by(8) {
                if let Some(seg) = jt.get_mut(i..i+8) {
                    // If segment is a load-segment trap instruction
                    if seg[2] == 0x3f && seg[3] == 0x3c && seg[6] == 0xa9 && seg[7] == 0xf0 {
                        let offset = (seg[0] as u32) << 8 | (seg[1] as u32);
                        let cur_id = ((seg[4] as u32) << 8 | (seg[5] as u32)) as i16;
                        if cur_id == id {
                            let new_address = offset + address;
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
        let name = self.rsrc.name(OSType::from(b"CODE"), id).ok()?.unwrap_or(PString::from("-"));

        // See if already loaded
        for (idx, (cur_id,_,  _)) in self.data.iter().enumerate() {
            if *cur_id == id {
                let address = idx as u32 * SEGMENT_MAX_SIZE + self.address_range.start();
                println!("Segment loader: already loaded: {} {} @{:08x}", id, name, address);
                return Some(address);
            } 
        }

        let idx = self.data.len();
        let address = idx as u32 * SEGMENT_MAX_SIZE + self.address_range.start();

        let mut data = self.rsrc.open(OSType::from(b"CODE"), id).ok()?;

        // The header is not included in the content
        let header = Header::read(&mut data, id).ok()?;

        let start_pos = data.pos() as usize;
        let data = data.to_vec().split_off(start_pos);

        self.data.push((id, header, data));

        println!("Segment loader: loading: {} {} @{:08x}", id, name, address);

        self.update_jump_table(id, address);

        Some(address)
    }
}

#[trap_handlers]
#[allow(non_snake_case)] // This function names comes from old Mac structs
impl SegmentLoader {
    #[trap(0xa9f0)]
    fn LoadSeg(&mut self, cpu: &mut CPU, code_id: i16) -> Option<()> {
        if let Some(_address) = self.load(code_id) {
            // The segment is loaded, jump back to the jump table
            // let pc = *core.pc();
            // core.jump(pc - 6);
            Some(())
        } else {
            println!("Unknown segment {}", code_id);
            None
        }
    }
}

impl CPUPeripheral for SegmentLoader {
    fn mem_read(&mut self, address: u32, size: usize) -> Option<&[u8]> {
        let segment_idx = address / SEGMENT_MAX_SIZE;
        let segment_offset = address % SEGMENT_MAX_SIZE;
        if let Some((_, _, data)) = self.data.get(segment_idx as usize) {
            // 4 bytes header on segment
            let segment_offset = segment_offset as usize;
            Some(&data[segment_offset..segment_offset+size])
        } else {
            None
        }
    }
    
    fn line_1010_emualtion(&mut self, cpu: &mut CPU, core: &mut CPUCore, ir: u16, pc: u32) -> Option<()> {
        self.trap_invoke(cpu, core, ir, pc)
    }
}