use super::{CPU, CPUCore};

pub trait CPUPeripheral {
    fn mem_read(&mut self, _address: u32, _size: usize) -> Option<&[u8]> {
        None
    }
    fn mem_write(&mut self, _address: u32, _data: &[u8]) -> Option<()> {
        None
    }
    fn line_1010_emualtion(&mut self, _cpu: &mut CPU, _core: &mut CPUCore, _ir: u16, _pc: u32) -> Option<()> {
        None
    }
}