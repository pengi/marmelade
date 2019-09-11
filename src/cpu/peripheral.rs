pub trait CPUPeripheral {
    fn mem_read(&mut self, _address: u32, _size: usize) -> Option<u32> {
        None
    }
    fn mem_write(&mut self, _address: u32, _data: u32, _size: usize) -> Option<()> {
        None
    }
    fn line_1010_emualtion(&mut self, _ir: u16, _pc: u32) -> Option<()> {
        None
    }
}