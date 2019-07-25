use std::io;
use std::fmt;

pub struct HfsVolBitmap (Vec<u8>);

impl fmt::Debug for HfsVolBitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut last = 0xff;
        let mut count = 0;
        write!(f, "HfsVolBitmap(")?;
        for b in self.0.iter() {
            if *b == last {
                count += 1;
            } else {
                write!(f, "{}*{:02x}, ", count, last)?;
                last = *b;
                count = 1;
            }
        }
        if count != 0 {
            write!(f, "{}*{:02x}", count, last)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

pub fn readvec(f : &mut io::Read, len : usize) -> io::Result<Vec<u8>> {
    let mut bufv : Vec<u8> = Vec::with_capacity(len);
    let mut bufa = [0u8; 1];

    for _i in 0..len {
        f.read_exact(&mut bufa)?;
        bufv.push(bufa[0]);
    }

    Ok(bufv)
}

impl HfsVolBitmap {
    pub fn from(file: &mut io::Read, num_blocks: u16) -> io::Result<HfsVolBitmap> {
        let bits_per_block = 512*8;
        let num_vbm_blocks = (num_blocks + bits_per_block - 1) / bits_per_block;
        let num_bytes = (num_vbm_blocks as usize) * 512;
        let data = readvec(file, num_bytes)?;
        Ok(HfsVolBitmap(data))
    }

    pub fn page_used(&self, block_num: u16) -> bool {
        let byte_idx = (block_num / 8) as usize;
        let bit_idx = 7-(block_num % 8);

        if let Some(b) = self.0.get(byte_idx) {
            *b & (1 << bit_idx) != 0
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HfsVolBitmap;
    #[test]
    fn page_used_bit_order() {
        let bm = HfsVolBitmap ( vec![0xffu8, 0xf0u8, 0x00u8] );
        assert_eq!(bm.page_used(0), true);
        assert_eq!(bm.page_used(7), true);
        assert_eq!(bm.page_used(8), true);
        assert_eq!(bm.page_used(15), false);
        assert_eq!(bm.page_used(16), false);
        assert_eq!(bm.page_used(23), false);
    }
}