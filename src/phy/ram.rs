use crate::cpu::{CPUPeripheral, AddressRange};

pub struct RAM {
    range: AddressRange,
    content: Vec<u8>
}

impl RAM {
    pub fn new(range: AddressRange) -> RAM {
        let size = range.size();
        RAM { range, content: vec![0xff; size] }
    }
}

impl CPUPeripheral for RAM {
    fn mem_read(&mut self, address: u32, size: usize) -> Option<&[u8]> {
        if let Some(mapped_address) = self.range.map(address, size) {
            let mapped_address = mapped_address as usize;
            Some(&self.content[mapped_address..mapped_address+size])
        } else {
            None
        }
    }
    
    fn mem_write(&mut self, address: u32, data: &[u8]) -> Option<()> {
        if let Some(mapped_address) = self.range.map(address, data.len()) {
            let mapped_address = mapped_address as usize;
            self.content.splice(
                mapped_address..mapped_address+data.len(),
                data.iter().cloned());
            Some(())
        } else {
            None
        }
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
        assert_eq!(Some(&[0x01u8, 0x23u8, 0x45u8, 0x67u8][..]), mem.mem_read(0x20000000, 4));
    }
    
    #[test]
    fn mem_read_u16() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        /* Force set values fo testing */
        mem.content[0] = 0x01;
        mem.content[1] = 0x23;
        mem.content[2] = 0x45;
        mem.content[3] = 0x67;
        assert_eq!(Some(&[0x01u8, 0x23u8][..]), mem.mem_read(0x20000000, 2));
        assert_eq!(Some(&[0x45u8, 0x67u8][..]), mem.mem_read(0x20000002, 2));
    }
    
    #[test]
    fn mem_read_u8() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        /* Force set values fo testing */
        mem.content[0] = 0x01;
        mem.content[1] = 0x23;
        mem.content[2] = 0x45;
        mem.content[3] = 0x67;
        assert_eq!(Some(&[0x45u8][..]), mem.mem_read(0x20000002, 1));
    }
    
    #[test]
    fn mem_write_u32() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        assert_eq!(Some(()), mem.mem_write(0x20001234, &[0xbau8, 0xddu8, 0xecu8, 0xafu8][..]));
        assert_eq!(Some(&[0xbau8, 0xddu8, 0xecu8, 0xafu8][..]), mem.mem_read(0x20001234, 4));
    }
    
    #[test]
    fn mem_write_u32_read_u16() {
        let mut mem = RAM::new(AddressRange::new_prefix(0x20000000, 16));
        assert_eq!(Some(()), mem.mem_write(0x20001234, &[0xbau8, 0xddu8, 0xecu8, 0xafu8][..]));
        assert_eq!(Some(&[0xbau8, 0xddu8][..]), mem.mem_read(0x20001234, 2));
        assert_eq!(Some(&[0xecu8, 0xafu8][..]), mem.mem_read(0x20001236, 2));
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
        assert_eq!(None, mem.mem_write(0x21000000, &[0x12u8, 0x34u8][..]));
    }
}