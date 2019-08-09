use r68k_emu::ram::{
    AddressBus,
    AddressSpace
};
use crate::phy::prefix::Prefix;
use super::Toolbox;
use std::rc::Weak;
use std::collections::HashMap;
use crate::types::{
    PString,
    OSType
};


const SEGMENT_MAX_SIZE : u32 = 0x8000;

pub struct SegmentLoader {
    address_base: u32,
    address_prefix: u32,
    toolbox: Weak<Toolbox>,
    data: Vec<(i16, Vec<u8>)>
}


impl SegmentLoader {
    pub fn new(address_base: u32, address_prefix: u32) -> SegmentLoader {
        SegmentLoader {
            address_base,
            address_prefix,
            toolbox: Weak::new(),
            data: vec![]
        }
    }

    pub fn get_prefix(&self) -> Prefix {
        Prefix::new(self.address_base, self.address_prefix)
    }

    pub fn set_toolbox(&mut self, toolbox: Weak<Toolbox>) {
        self.toolbox = toolbox;
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

            let data = toolbox.rsrc.open(OSType::from(b"CODE"), id).ok()?.to_vec();
            self.data.push((id, data));

            println!("Segment loader: loading: {} {} @{:08x}", id, name, address);
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
            *data.get(segment_offset as usize + 4).unwrap_or(&0xff) as u32
        } else {
            0xff
        }
    }
    fn write_byte(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        println!("Can't write to segment {:08x}", address);
    }




    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
        (self.read_byte(address_space, address) << 8) | self.read_byte(address_space, address+1)
    }
    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
        (self.read_word(address_space, address) << 16) | self.read_word(address_space, address+2)
    }
    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        self.write_byte(address_space, address+0, (value>>8)&0xff);
        self.write_byte(address_space, address+1, (value>>0)&0xff);
    }
    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        self.write_word(address_space, address+0, (value>>16)&0xffff);
        self.write_word(address_space, address+2, (value>>0)&0xffff);
    }
}