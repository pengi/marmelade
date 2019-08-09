use r68k_emu::ram::{
    AddressBus,
    AddressSpace
};
use crate::phy::prefix::Prefix;
use super::Toolbox;
use std::rc::Weak;

pub struct SegmentLoader {
    address_base: u32,
    address_prefix: u32,
    toolbox: Weak<Toolbox>
}


impl SegmentLoader {
    pub fn new(address_base: u32, address_prefix: u32) -> SegmentLoader {
        SegmentLoader {
            address_base,
            address_prefix,
            toolbox: Weak::new()
        }
    }

    pub fn get_prefix(&self) -> Prefix {
        Prefix::new(self.address_base, self.address_prefix)
    }

    pub fn set_toolbox(&mut self, toolbox: Weak<Toolbox>) {
        self.toolbox = toolbox;
    }
}

impl AddressBus for SegmentLoader {
    fn read_byte(&self, _address_space: AddressSpace, address: u32) -> u32 {
        0xff
    }
    fn write_byte(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
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