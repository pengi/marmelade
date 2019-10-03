use std::vec::Vec;
use std::cell::RefCell;
use super::{CPU, CPUPeripheral, CPUCore};

fn vec_to_u32(bytes: &[u8]) -> u32 {
    let mut result = 0;
    for value in bytes {
        result <<= 8;
        result |= *value as u32;
    }
    result
}

fn u32_to_vec(bytes: &mut[u8], value: u32) {
    let mut value = value;
    for byte in bytes.iter_mut().rev() {
        *byte = (value & 0xff) as u8;
        value >>= 8;
    }
}

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
                    return Some(vec_to_u32(result));
                }
            }
        }
        None
    }
    pub fn mem_write(&self, address: u32, data: u32, size: usize) -> Option<()> {
        let mut bytes = [0u8; 4];
        u32_to_vec(&mut bytes[0..size], data);
        
        for elem in self.peripherals.iter() {
            if let Some(mut p) = elem.try_borrow_mut().ok() {
                if let Some(_) = p.mem_write(address, &bytes[0..size]) {
                    return Some(());
                }
            }
        }
        None
    }
    pub fn line_1010_emualtion(&self, cpu: &mut CPU, core: &mut CPUCore, ir: u16, pc: u32) -> Option<()> {
        for elem in self.peripherals.iter() {
            if let Some(mut p) = elem.try_borrow_mut().ok() {
                if let Some(_) = p.line_1010_emualtion(cpu, core, ir, pc) {
                    return Some(());
                }
            }
        }
        None
    }
}