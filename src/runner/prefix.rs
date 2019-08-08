
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
    pub value : u32
}

impl Prefix {
    pub fn new(value : u32, length : u32) -> Prefix {
        let mask = PREFIX_ONES[length as usize];
        assert_eq!(0, value & !mask);
        Prefix { mask, value }
    }

    pub fn contains_value(&self, value: u32) -> bool {
        (value & self.mask) == self.value
    }

    pub fn contains_prefix(&self, value: Prefix) -> bool {
        if (self.mask & !value.mask) != 0 {
            // A longer prefix can't contain a shorter
            false
        } else {
            // ...otherwise check if first value contains
            (value.value & self.mask) == self.value
        }
    }
}

pub struct PrefixMap<T> {
    // Keep a simple implementation for now
    children: Vec<(Prefix, T)>
}

impl<T> From<Vec<(Prefix, T)>> for PrefixMap<T> {
    fn from(vec: Vec<(Prefix, T)>) -> PrefixMap<T> {
        PrefixMap {
            children: vec
        }
    }
}

impl<T> PrefixMap<T> {
    fn locate(&self, address : u32) -> Option<&T> {
        for (prefix, ref value) in self.children.iter() {
            if prefix.contains_value(address) {
                return Some(value);
            }
        }
        None
    }
    fn locate_mut(&mut self, address : u32) -> Option<&mut T> {
        for (prefix, ref mut value) in self.children.iter_mut() {
            if prefix.contains_value(address) {
                return Some(value);
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::{
        Prefix,
        PrefixMap
    };

    #[test]
    fn address_prefix_mask() {
        let prefix = Prefix::new(0x43000000, 8);
        assert_eq!(prefix, Prefix{
            mask: 0xff000000,
            value: 0x43000000
        });

        let prefix = Prefix::new(0x00000000, 0);
        assert_eq!(prefix, Prefix{
            mask: 0x00000000,
            value: 0x00000000
        });

        let prefix = Prefix::new(0x12345678, 32);
        assert_eq!(prefix, Prefix{
            mask: 0xffffffff,
            value: 0x12345678
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
    fn locate_map() {
        let map = PrefixMap::from(vec![
            (Prefix::new(0x00000000, 20), 1),
            (Prefix::new(0x00001000, 20), 2),
            (Prefix::new(0x20000000, 8), 3),
            (Prefix::new(0x40000000, 8), 4)
        ]);
        assert_eq!(map.locate(0x00000123), Some(&1));
        assert_eq!(map.locate(0x00001000), Some(&2));
        assert_eq!(map.locate(0x00001123), Some(&2));
        assert_eq!(map.locate(0x00003123), None);
        assert_eq!(map.locate(0x10003123), None);
        assert_eq!(map.locate(0x20003123), Some(&3));
        assert_eq!(map.locate(0x30003123), None);
    }

    #[test]
    fn locate_mut_map() {
        let mut map = PrefixMap::from(vec![
            (Prefix::new(0x00000000, 20), 1),
            (Prefix::new(0x00001000, 20), 2),
            (Prefix::new(0x20000000, 8), 3),
            (Prefix::new(0x40000000, 8), 4)
        ]);
        assert_eq!(map.locate_mut(0x00000123), Some(&mut 1));
        assert_eq!(map.locate_mut(0x00001000), Some(&mut 2));
        assert_eq!(map.locate_mut(0x00001123), Some(&mut 2));
        assert_eq!(map.locate_mut(0x00003123), None);
        assert_eq!(map.locate_mut(0x10003123), None);
        assert_eq!(map.locate_mut(0x20003123), Some(&mut 3));
        assert_eq!(map.locate_mut(0x30003123), None);
    }
}