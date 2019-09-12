use crate::cpu::{CPUPeripheral, AddressRange};

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

pub struct RAM {
    range: AddressRange,
    content: Vec<u8>
}

impl RAM {
    pub fn new(range: AddressRange) -> RAM {
        let size = range.size();
        RAM { range, content: vec![0xff; size] }
    }
    
    fn apply<R, F: FnOnce(&mut [u8]) -> R>(&mut self, address: u32, size: usize, op: F) -> Option<R> {
        if let Some(mapped_address) = self.range.map(address, size) {
            let mapped_address = mapped_address as usize;
            let bytes = &mut self.content[mapped_address..mapped_address+size];
            Some(op(bytes))
        } else {
            None
        }
    }
}

impl CPUPeripheral for RAM {
    fn mem_read(&mut self, address: u32, size: usize) -> Option<u32> {
        self.apply(address, size, |bytes| vec_to_u32(bytes))
    }
    
    fn mem_write(&mut self, address: u32, data: u32, size: usize) -> Option<()> {
        self.apply(address, size, |bytes| u32_to_vec(bytes, data))
    }
}



#[cfg(test)]
mod tests {
    use super::{
        RAM,
        CPUPeripheral,
        AddressRange
    };
    
    #[test]
    fn mem_read_u32() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        /* Force set values fo testing */
        mem.content[0] = 0x01;
        mem.content[1] = 0x23;
        mem.content[2] = 0x45;
        mem.content[3] = 0x67;
        assert_eq!(Some(0x01234567), mem.mem_read(0x20000000, 4));
    }
    
    #[test]
    fn mem_read_u16() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        /* Force set values fo testing */
        mem.content[0] = 0x01;
        mem.content[1] = 0x23;
        mem.content[2] = 0x45;
        mem.content[3] = 0x67;
        assert_eq!(Some(0x0123), mem.mem_read(0x20000000, 2));
        assert_eq!(Some(0x4567), mem.mem_read(0x20000002, 2));
    }
    
    #[test]
    fn mem_read_u8() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        /* Force set values fo testing */
        mem.content[0] = 0x01;
        mem.content[1] = 0x23;
        mem.content[2] = 0x45;
        mem.content[3] = 0x67;
        assert_eq!(Some(0x45), mem.mem_read(0x20000002, 1));
    }
    
    #[test]
    fn mem_write_u32() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        assert_eq!(Some(()), mem.mem_write(0x20001234, 0xbaddecaf, 4));
        assert_eq!(Some(0xbaddecaf), mem.mem_read(0x20001234, 4));
    }
    
    #[test]
    fn mem_write_u32_read_u16() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        assert_eq!(Some(()), mem.mem_write(0x20001234, 0xbaddecaf, 4));
        assert_eq!(Some(0xbadd), mem.mem_read(0x20001234, 2));
        assert_eq!(Some(0xecaf), mem.mem_read(0x20001236, 2));
    }
    
    #[test]
    fn mem_read_over_end() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        assert_eq!(None, mem.mem_read(0x2000fffe, 4));
    }
    
    #[test]
    fn mem_read_out_of_block() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        assert_eq!(None, mem.mem_read(0x21000000, 4));
    }
    
    #[test]
    fn mem_write_out_of_block() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        assert_eq!(None, mem.mem_write(0x21000000, 313, 4));
    }
}