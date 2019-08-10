use r68k_emu::ram::{
    AddressBus,
    AddressSpace
};

pub struct RAM {
    content: Vec<u8>
}

impl RAM {
    pub fn new(size: usize) -> RAM {
        RAM { content: vec![0x00; size] }
    }
}

impl From<Vec<u8>> for RAM {
    fn from(vec: Vec<u8>) -> RAM {
        RAM { content: vec }
    }
}

impl AddressBus for RAM {
    fn read_byte(&self, _address_space: AddressSpace, address: u32) -> u32 {
        if let Some(value) = self.content.get(address as usize) {
            *value as u32
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
        if let Some(ptr) = self.content.get_mut(address as usize) {
            *ptr = value as u8
        }
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