
use r68k_emu::ram::AddressBus;
use r68k_emu::ram::AddressSpace;


pub struct MuxAddressBus;

impl AddressBus for MuxAddressBus {
    fn copy_from(&mut self, _other: &Self) {
        unimplemented!();
    }

    fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32 {
        println!("read_byte({:?}, {:08x})", address_space, address);
        0xff
    }
    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
        println!("read_word({:?}, {:08x})", address_space, address);
        0xffff
    }
    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
        let val = match address {
            // 0x00001000 => 0xd03c_0003,
            // 0x00001004 => 0xd200_ffff,
            0x00001000 => 0x3F3C_0001,
            0x00001004 => 0xA9F0_ffff,
            _ => 0xffffffff
        };
        println!("read_long({:?}, {:08x}) = {:08x}", address_space, address, val);
        val
    }
    fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        println!("write_byte({:?}, {:08x}, {:02x})", address_space, address, value);

    }
    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        println!("write_word({:?}, {:08x}, {:04x})", address_space, address, value);

    }
    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        println!("write_long({:?}, {:08x}, {:08x})", address_space, address, value);
        
    }
}