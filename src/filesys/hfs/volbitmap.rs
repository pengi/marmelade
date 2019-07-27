use super::fileadaptor::FileBlock;
use std::fmt;

pub struct HfsVolBitmap(FileBlock);

impl fmt::Debug for HfsVolBitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HfsVolBitmap()")?;
        Ok(())
    }
}

impl HfsVolBitmap {
    pub fn from(block: FileBlock) -> HfsVolBitmap {
        HfsVolBitmap(block)
    }

    pub fn page_used(&self, block_num: u16) -> bool {
        let byte_idx = (block_num / 8) as usize;
        let bit_idx = 7 - (block_num % 8);

        self.0.read_u8(byte_idx) & (1 << bit_idx) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::FileBlock;
    use super::HfsVolBitmap;

    use super::super::fileadaptor::FileAdaptor;
    use super::super::fileadaptor::FileAccess;

    fn block_for_vec(vec : Vec<u8>) -> FileBlock {
        let len = vec.len();
        let mut io = std::io::Cursor::new(vec);
        let mut fa = FileAdaptor::new(&mut io);
        fa.read_vec(len).unwrap()
    }

    #[test]
    fn page_used_bit_order() {
        let bm = HfsVolBitmap(block_for_vec(vec![0xffu8, 0xf0u8, 0x00u8]));
        assert_eq!(bm.page_used(0), true);
        assert_eq!(bm.page_used(7), true);
        assert_eq!(bm.page_used(8), true);
        assert_eq!(bm.page_used(15), false);
        assert_eq!(bm.page_used(16), false);
        assert_eq!(bm.page_used(23), false);
    }
}
