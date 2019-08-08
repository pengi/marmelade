use r68k_emu::ram::{
    AddressBus,
    AddressSpace
};

pub struct ROM {
    content: Vec<u8>
}

impl From<Vec<u8>> for ROM {
    fn from(vec: Vec<u8>) -> ROM {
        ROM { content: vec }
    }
}

impl AddressBus for ROM {
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
    fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        println!("write to ROM: write_byte({:?}, {:08x}, {:02x})", address_space, address, value);
    }
    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        println!("write to ROM: write_word({:?}, {:08x}, {:04x})", address_space, address, value);
    }
    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        println!("write to ROM: write_long({:?}, {:08x}, {:08x})", address_space, address, value);
    }
}