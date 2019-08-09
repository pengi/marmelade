use r68k_emu::ram::{
    AddressBus,
    AddressSpace
};

use std::rc::Rc;
use std::cell::{
    RefCell,
    RefMut
};
use std::ops::Deref;

pub struct RcMem<M : AddressBus> {
    child: Rc<RefCell<M>>
}

impl<M : AddressBus> AddressBus for RcMem<M> {
    fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32 {
        let mem = self.child.deref().borrow();
        mem.read_byte(address_space, address)
    }
    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
        let mem = self.child.deref().borrow();
        mem.read_word(address_space, address)
    }
    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
        let mem = self.child.deref().borrow();
        mem.read_long(address_space, address)
    }
    fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        let mut mem = self.child.deref().borrow_mut();
        mem.write_byte(address_space, address, value)
    }
    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        let mut mem = self.child.deref().borrow_mut();
        mem.write_word(address_space, address, value)
    }
    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        let mut mem = self.child.deref().borrow_mut();
        mem.write_long(address_space, address, value)
    }
}

impl<M : AddressBus> Clone for RcMem<M> {
    fn clone(&self) -> RcMem<M> {
        RcMem {
            child: self.child.clone()
        }
    }
}

impl<M : AddressBus> RcMem<M> {
    pub fn new(sub: M) -> RcMem<M> {
        RcMem {
            child: Rc::new(RefCell::new(sub))
        }
    }

    pub fn borrow_mut(&self) -> RefMut<M> {
        self.child.deref().borrow_mut()
    }
}