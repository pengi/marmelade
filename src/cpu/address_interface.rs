use super::CPUBus;
use r68k_emu::ram::{
    AddressBus,
    AddressSpace
};
use std::rc::Rc;
use std::cell::{RefCell, Ref};

pub struct CPUAddressInterface {
    peripheral: Rc<RefCell<CPUBus>>
}

impl CPUAddressInterface {
    pub fn new(peripheral: &Rc<RefCell<CPUBus>>) -> CPUAddressInterface {
        CPUAddressInterface {
            peripheral: Rc::clone(peripheral)
        }
    }
}

impl AddressBus for CPUAddressInterface {
    fn read_byte(&self, _address_space: AddressSpace, address: u32) -> u32 {
        let p: Ref<_> = self.peripheral.borrow();
        p.mem_read(address, 1).unwrap_or_default()
    }
    fn read_word(&self, _address_space: AddressSpace, address: u32) -> u32 {
        let p: Ref<_> = self.peripheral.borrow();
        p.mem_read(address, 2).unwrap_or_default()
    }
    fn read_long(&self, _address_space: AddressSpace, address: u32) -> u32 {
        let p: Ref<_> = self.peripheral.borrow();
        p.mem_read(address, 4).unwrap_or_default()
    }
    fn write_byte(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        let p: Ref<_> = self.peripheral.borrow();
        p.mem_write(address, value, 1);
    }
    fn write_word(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        let p: Ref<_> = self.peripheral.borrow();
        p.mem_write(address, value, 2);
    }
    fn write_long(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        let p: Ref<_> = self.peripheral.borrow();
        p.mem_write(address, value, 4);
    }
}

