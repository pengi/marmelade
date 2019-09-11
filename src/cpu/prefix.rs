
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
pub struct Prefix {
    pub mask : u32,
    pub address : u32
}

impl Prefix {
    pub fn new(address : u32, length : u32) -> Prefix {
        let mask = PREFIX_ONES[length as usize];
        assert_eq!(0, address & !mask);
        Prefix { mask, address }
    }

    pub fn contains_value(&self, address: u32) -> bool {
        (address & self.mask) == self.address
    }

    pub fn contains_prefix(&self, other: Prefix) -> bool {
        if (self.mask & !other.mask) != 0 {
            // A longer prefix can't contain a shorter
            false
        } else {
            // ...otherwise check if first value contains
            (other.address & self.mask) == self.address
        }
    }
    
    pub fn map(&self, address: u32, size: usize) -> Option<u32> {
        if self.contains_value(address) && self.contains_value(address + (size as u32) - 1) {
            Some(address & !self.mask)
        } else {
            None
        }
    }
    
    pub fn size(&self) -> usize {
        1 + !self.mask as usize
    }
}


#[cfg(test)]
mod tests {
    use super::{
        Prefix
    };

    #[test]
    fn address_prefix_mask() {
        let prefix = Prefix::new(0x43000000, 8);
        assert_eq!(prefix, Prefix{
            mask: 0xff000000,
            address: 0x43000000
        });

        let prefix = Prefix::new(0x00000000, 0);
        assert_eq!(prefix, Prefix{
            mask: 0x00000000,
            address: 0x00000000
        });

        let prefix = Prefix::new(0x12345678, 32);
        assert_eq!(prefix, Prefix{
            mask: 0xffffffff,
            address: 0x12345678
        });
    }

    #[test]
    #[should_panic]
    fn address_prefix_masked_should_be_zero() {
        let _prefix = Prefix::new(0x12345678, 16);
    }

    #[test]
    #[should_panic]
    fn address_prefix_too_long_mask() {
        let _prefix = Prefix::new(0x12345678, 33);
    }

    #[test]
    fn address_prefix_contains_value() {
        assert!(Prefix::new(0x43000000, 8).contains_value(0x43001010));
        assert!(Prefix::new(0x43000000, 8).contains_value(0x43000000));
        assert!(Prefix::new(0x43000000, 8).contains_value(0x43ffffff));
        assert!(Prefix::new(0x43000000, 8).contains_value(0x43100000));
        
        assert!(Prefix::new(0x43000000, 9).contains_value(0x43001010));
        assert!(Prefix::new(0x43000000, 9).contains_value(0x43000000));
        assert!(Prefix::new(0x43000000, 9).contains_value(0x437fffff));
        assert!(!Prefix::new(0x43000000, 9).contains_value(0x43ffffff));
        assert!(!Prefix::new(0x43000000, 9).contains_value(0x43800000));
    }

    #[test]
    fn address_prefix_contains_prefix() {
        assert!(Prefix::new(0x43000000, 8).contains_prefix(Prefix::new(0x43001010,32)));
        assert!(Prefix::new(0x43000000, 8).contains_prefix(Prefix::new(0x43000000,32)));
        assert!(Prefix::new(0x43000000, 8).contains_prefix(Prefix::new(0x43ffffff,32)));
        assert!(Prefix::new(0x43000000, 8).contains_prefix(Prefix::new(0x43100000,32)));
        assert!(Prefix::new(0x43000000, 9).contains_prefix(Prefix::new(0x43001010,32)));
        assert!(Prefix::new(0x43000000, 9).contains_prefix(Prefix::new(0x43000000,32)));
        assert!(Prefix::new(0x43000000, 9).contains_prefix(Prefix::new(0x437fffff,32)));
        assert!(!Prefix::new(0x43000000, 9).contains_prefix(Prefix::new(0x43ffffff,32)));
        assert!(!Prefix::new(0x43000000, 9).contains_prefix(Prefix::new(0x43800000,32)));

        assert!(!Prefix::new(0x12340000, 24).contains_prefix(Prefix::new(0x12340000,16)));
    }
    
    #[test]
    fn map_address() {
        assert_eq!(Prefix::new(0x43000000, 9).map(0x43000000, 4), Some(0));
        assert_eq!(Prefix::new(0x43000000, 9).map(0x43123456, 4), Some(0x123456));
        assert_eq!(Prefix::new(0x43000000, 9).map(0x437fffff, 1), Some(0x7fffff));
        assert_eq!(Prefix::new(0x43000000, 9).map(0x437fffff, 2), None);
        assert_eq!(Prefix::new(0x43000000, 9).map(0x437fffff, 4), None);
        assert_eq!(Prefix::new(0x43000000, 9).map(0x43ffffff, 1), None);
        assert_eq!(Prefix::new(0x43000000, 9).map(0x43800000, 4), None);
    }
    
    #[test]
    fn prefix_size() {
        assert_eq!(Prefix::new(0x43000000, 9).size(), 0x0080_0000);
        assert_eq!(Prefix::new(0x43000000, 16).size(), 0x0001_0000);
        assert_eq!(Prefix::new(0x43000000, 32).size(), 0x0000_0001);
        assert_eq!(Prefix::new(0x80000000, 1).size(), 0x8000_0000);
    }
}