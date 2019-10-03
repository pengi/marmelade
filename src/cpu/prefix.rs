
const PREFIX_ONES : [u32; 33] = [
    0x0000_0000, 0x8000_0000, 0xc000_0000, 0xe000_0000,
    0xf000_0000, 0xf800_0000, 0xfc00_0000, 0xfe00_0000,
    0xff00_0000, 0xff80_0000, 0xffc0_0000, 0xffe0_0000,
    0xfff0_0000, 0xfff8_0000, 0xfffc_0000, 0xfffe_0000,
    0xffff_0000, 0xffff_8000, 0xffff_c000, 0xffff_e000,
    0xffff_f000, 0xffff_f800, 0xffff_fc00, 0xffff_fe00,
    0xffff_ff00, 0xffff_ff80, 0xffff_ffc0, 0xffff_ffe0,
    0xffff_fff0, 0xffff_fff8, 0xffff_fffc, 0xffff_fffe,
    0xffff_ffff
];

#[derive(PartialEq, Debug)]
pub struct AddressRange {
    // Points to to first byte in range
    address_start : u32,
    
    // Points to last byte included in range, to allow for 0xffffffff
    address_end : u32
}

impl AddressRange {
    pub fn new_prefix(address : u32, length : u32) -> AddressRange {
        let mask = PREFIX_ONES[length as usize];
        assert_eq!(0, address & !mask);
        AddressRange {
            address_start: address,
            address_end: address | !mask
        }
    }
    
    pub fn new(address_start : u32, address_end : u32) -> AddressRange {
        assert!(address_start <= address_end);
        AddressRange {
            address_start: address_start,
            address_end: address_end
        }
    }

    pub fn contains_value(&self, address: u32) -> bool {
        (address >= self.address_start) && (address <= self.address_end)
    }

    pub fn contains_range(&self, other: AddressRange) -> bool {
        (other.address_start >= self.address_start) && (other.address_end <= self.address_end)
    }
    
    pub fn map(&self, address: u32, size: usize) -> Option<u32> {
        if (address >= self.address_start) && ((address + size as u32 - 1) <= self.address_end) {
            Some(address - self.address_start)
        } else {
            None
        }
    }
    
    pub fn start(&self) -> u32 {
        self.address_start
    }
    
    pub fn size(&self) -> usize {
        (self.address_end - self.address_start + 1) as usize
    }
}


#[cfg(test)]
mod tests {
    use super::{
        AddressRange
    };

    #[test]
    fn address_range_mask() {
        let range = AddressRange::new_prefix(0x43000000, 8);
        assert_eq!(range, AddressRange{
            address_start: 0x43000000,
            address_end: 0x43ffffff
        });

        let range = AddressRange::new_prefix(0x00000000, 0);
        assert_eq!(range, AddressRange{
            address_start: 0x00000000,
            address_end: 0xffffffff
        });

        let range = AddressRange::new_prefix(0x12345678, 32);
        assert_eq!(range, AddressRange{
            address_start: 0x12345678,
            address_end: 0x12345678
        });
    }

    #[test]
    #[should_panic]
    fn address_range_masked_should_be_zero() {
        let _range = AddressRange::new_prefix(0x12345678, 16);
    }

    #[test]
    #[should_panic]
    fn address_range_too_long_mask() {
        let _range = AddressRange::new_prefix(0x12345678, 33);
    }

    #[test]
    fn address_range_contains_value() {
        assert!(AddressRange::new_prefix(0x43000000, 8).contains_value(0x43001010));
        assert!(AddressRange::new_prefix(0x43000000, 8).contains_value(0x43000000));
        assert!(AddressRange::new_prefix(0x43000000, 8).contains_value(0x43ffffff));
        assert!(AddressRange::new_prefix(0x43000000, 8).contains_value(0x43100000));
        
        assert!(AddressRange::new_prefix(0x43000000, 9).contains_value(0x43001010));
        assert!(AddressRange::new_prefix(0x43000000, 9).contains_value(0x43000000));
        assert!(AddressRange::new_prefix(0x43000000, 9).contains_value(0x437fffff));
        assert!(!AddressRange::new_prefix(0x43000000, 9).contains_value(0x43ffffff));
        assert!(!AddressRange::new_prefix(0x43000000, 9).contains_value(0x43800000));
    }

    #[test]
    fn address_range_contains_range() {
        assert!(AddressRange::new_prefix(0x43000000, 8).contains_range(AddressRange::new_prefix(0x43001010,32)));
        assert!(AddressRange::new_prefix(0x43000000, 8).contains_range(AddressRange::new_prefix(0x43000000,32)));
        assert!(AddressRange::new_prefix(0x43000000, 8).contains_range(AddressRange::new_prefix(0x43ffffff,32)));
        assert!(AddressRange::new_prefix(0x43000000, 8).contains_range(AddressRange::new_prefix(0x43100000,32)));
        assert!(AddressRange::new_prefix(0x43000000, 9).contains_range(AddressRange::new_prefix(0x43001010,32)));
        assert!(AddressRange::new_prefix(0x43000000, 9).contains_range(AddressRange::new_prefix(0x43000000,32)));
        assert!(AddressRange::new_prefix(0x43000000, 9).contains_range(AddressRange::new_prefix(0x437fffff,32)));
        assert!(!AddressRange::new_prefix(0x43000000, 9).contains_range(AddressRange::new_prefix(0x43ffffff,32)));
        assert!(!AddressRange::new_prefix(0x43000000, 9).contains_range(AddressRange::new_prefix(0x43800000,32)));

        assert!(!AddressRange::new_prefix(0x12340000, 24).contains_range(AddressRange::new_prefix(0x12340000,16)));
    }
    
    #[test]
    fn map_address() {
        assert_eq!(AddressRange::new_prefix(0x43000000, 9).map(0x43000000, 4), Some(0));
        assert_eq!(AddressRange::new_prefix(0x43000000, 9).map(0x43123456, 4), Some(0x123456));
        assert_eq!(AddressRange::new_prefix(0x43000000, 9).map(0x437fffff, 1), Some(0x7fffff));
        assert_eq!(AddressRange::new_prefix(0x43000000, 9).map(0x437fffff, 2), None);
        assert_eq!(AddressRange::new_prefix(0x43000000, 9).map(0x437fffff, 4), None);
        assert_eq!(AddressRange::new_prefix(0x43000000, 9).map(0x43ffffff, 1), None);
        assert_eq!(AddressRange::new_prefix(0x43000000, 9).map(0x43800000, 4), None);
    }
    
    #[test]
    fn range_size() {
        assert_eq!(AddressRange::new_prefix(0x43000000, 9).size(), 0x0080_0000);
        assert_eq!(AddressRange::new_prefix(0x43000000, 16).size(), 0x0001_0000);
        assert_eq!(AddressRange::new_prefix(0x43000000, 32).size(), 0x0000_0001);
        assert_eq!(AddressRange::new_prefix(0x80000000, 1).size(), 0x8000_0000);
    }
}