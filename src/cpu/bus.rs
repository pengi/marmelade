use std::vec::Vec;
use std::cell::RefCell;
use super::CPUPeripheral;

pub struct CPUBus {
    peripherals: Vec<RefCell<Box<dyn CPUPeripheral>>>
}

impl CPUBus {
    pub fn new() -> CPUBus {
        CPUBus {
            peripherals: vec![]
        }
    }
    
    pub fn attach(&mut self, peripheral: Box<dyn CPUPeripheral>) {
        self.peripherals.push(RefCell::new(peripheral))
    }
    
    pub fn mem_read(&self, address: u32, size: usize) -> Option<u32> {
        for elem in self.peripherals.iter() {
            if let Some(mut p) = elem.try_borrow_mut().ok() {
                if let Some(result) = p.mem_read(address, size) {
                    return Some(result);
                }
            }
        }
        None
    }
    pub fn mem_write(&self, address: u32, data: u32, size: usize) -> Option<()> {
        for elem in self.peripherals.iter() {
            if let Some(mut p) = elem.try_borrow_mut().ok() {
                if let Some(_) = p.mem_write(address, data, size) {
                    return Some(());
                }
            }
        }
        None
    }
    pub fn line_1010_emualtion(&self, ir: u16, pc: u32) -> Option<()> {
        for elem in self.peripherals.iter() {
            if let Some(mut p) = elem.try_borrow_mut().ok() {
                if let Some(_) = p.line_1010_emualtion(ir, pc) {
                    return Some(());
                }
            }
        }
        None
    }
}