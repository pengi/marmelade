
use r68k_emu::ram::{
    AddressBus,
    AddressSpace
};

use super::super::prefix::{
    Prefix,
    PrefixMap
};

pub struct MuxMem {
    children: PrefixMap<Box<dyn AddressBus>>
}

impl AddressBus for MuxMem {
    fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32 {
        if let Some((address, bus)) = self.children.locate(address) {
            bus.read_byte(address_space, address)
        } else {
            println!("unmapped read_byte({:?}, {:08x})", address_space, address);
            0xff
        }
    }
    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
        if let Some((address, bus)) = self.children.locate(address) {
            bus.read_word(address_space, address)
        } else {
            println!("unmapped read_word({:?}, {:08x})", address_space, address);
            0xffff
        }
    }
    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
        if let Some((address, bus)) = self.children.locate(address) {
            bus.read_long(address_space, address)
        } else {
            println!("unmapped read_long({:?}, {:08x})", address_space, address);
            0xffffffff
        }
    }
    fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        if let Some((address, bus)) = self.children.locate_mut(address) {
            bus.write_byte(address_space, address, value);
        } else {
            println!("unmapped write_byte({:?}, {:08x}, {:02x})", address_space, address, value);
        }
    }
    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        if let Some((address, bus)) = self.children.locate_mut(address) {
            bus.write_word(address_space, address, value);
        } else {
            println!("unmapped write_word({:?}, {:08x}, {:04x})", address_space, address, value);
        }

    }
    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        if let Some((address, bus)) = self.children.locate_mut(address) {
            bus.write_long(address_space, address, value);
        } else {
            println!("unmapped write_long({:?}, {:08x}, {:08x})", address_space, address, value);
        }
        
    }
}

impl MuxMem {
    pub fn new() -> MuxMem {
        MuxMem {
            children: PrefixMap::from(vec![])
        }
    }

    pub fn add_prefix(&mut self, prefix: Prefix, bus: Box<dyn AddressBus>) {
        self.children.add_prefix(prefix, bus);
    }
}