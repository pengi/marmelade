use crate::serialization::{
    SerialAccess,
    SerialReadStorage
};

use std::rc::Rc;
use std::borrow::Borrow;

use super::types::common::{
    ExtDataRec,
    ExtDescriptor
};

#[derive(Debug)]
#[derive(Clone)]
pub struct BlockAccess {
    storage: Rc<Box<dyn SerialAccess>>,
    alblk_start: u64,
    alblk_size: u64
}

impl BlockAccess {
    pub fn new(storage: Box<dyn SerialAccess>, alblk_start: u64, alblk_size: u64) -> BlockAccess {
        BlockAccess {
            storage: Rc::new(storage),
            alblk_start: alblk_start * 512,
            alblk_size
        }
    }

    fn do_read_blk(&self, offset: u64, len: u64) -> std::io::Result<SerialReadStorage> {
        let storage : &Box<dyn SerialAccess> = self.storage.borrow();
        storage.read(offset, len)
    }

    fn do_read_extdescriptor(&self, descr: &ExtDescriptor, offset: u64, len: u64) -> std::io::Result<SerialReadStorage> {
        self.do_read_blk(
            self.alblk_start + (descr.xdrStABN as u64) * self.alblk_size + offset,
            len)
    }

    pub fn read_extdatarec(&self, rec : &ExtDataRec, offset : u64, len : u64) -> std::io::Result<SerialReadStorage> {
        let mut left_offset = offset;
        let mut left_len = len;
        let mut output : SerialReadStorage = SerialReadStorage::from(vec![]); 

        for rec in rec.0.iter() {
            let rec_size = rec.xdrNumABlks as u64 * self.alblk_size;

            if left_offset >= rec_size {
                // Starts after block, skip
                left_offset -= rec_size;
            } else if left_offset+left_len > rec_size {
                // Overlaps end, skip
                let take_len = rec_size - left_offset;
                output.extend(self.do_read_extdescriptor(rec, left_offset, take_len)?);

                left_len -= take_len;
                left_offset = 0;
            } else {
                // Contains in block, use and break
                output.extend(self.do_read_extdescriptor(rec, left_offset, left_len)?);

                break;
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SerialAccess,
        BlockAccess,
        ExtDataRec,
        ExtDescriptor,
        Rc,
        SerialReadStorage
    };

    #[derive(Debug)]
    struct MockDisk {
        size : u64
    }

    impl SerialAccess for MockDisk {
        fn size(&self) -> std::io::Result<u64> {
            Ok(self.size)
        }
        fn read(&self, pos : u64, len : u64) -> std::io::Result<SerialReadStorage> {
            if pos + len >= self.size {
                Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))
            } else {
                let output : Vec<u8> = (pos as u8..(pos + len) as u8).collect();
                Ok(SerialReadStorage::from(output))
            }
        }
    }

    fn mock_ba(size : u64, blocksize : u64) -> BlockAccess {
        BlockAccess {
            storage: Rc::new(Box::new(MockDisk {
                size: size
            })),
            alblk_size: blocksize,
            alblk_start: 0
        }
    }

    #[test]
    fn read_ext_single_block() -> std::io::Result<()> {
        let ba = mock_ba(50,8);

        let datarec = ExtDataRec ([
            ExtDescriptor { xdrStABN: 1, xdrNumABlks: 1 },
            ExtDescriptor { xdrStABN: 0, xdrNumABlks: 0 },
            ExtDescriptor { xdrStABN: 0, xdrNumABlks: 0 },
        ]);
        assert_eq!(
            ba.read_extdatarec(&datarec, 0, 8)?.to_vec(),
            [8,9,10,11,12,13,14,15]
        );
        Ok(())
    }

    #[test]
    fn read_ext_multi_block_offset() -> std::io::Result<()> {
        let ba = mock_ba(50,4);

        let datarec = ExtDataRec ([
            ExtDescriptor { xdrStABN: 2, xdrNumABlks: 3 },
            ExtDescriptor { xdrStABN: 0, xdrNumABlks: 0 },
            ExtDescriptor { xdrStABN: 0, xdrNumABlks: 0 },
        ]);
        assert_eq!(
            ba.read_extdatarec(&datarec, 2, 8)?.to_vec(),
            [10,11,12,13,14,15,16,17]
        );
        Ok(())
    }

    #[test]
    fn read_ext_no_continous_block() -> std::io::Result<()> {
        let ba = mock_ba(50,4);

        let datarec = ExtDataRec ([
            ExtDescriptor { xdrStABN: 1, xdrNumABlks: 1 },
            ExtDescriptor { xdrStABN: 0, xdrNumABlks: 1 },
            ExtDescriptor { xdrStABN: 2, xdrNumABlks: 1 },
        ]);
        assert_eq!(
            ba.read_extdatarec(&datarec, 2, 8)?.to_vec(),
            [6,7,0,1,2,3,8,9]
        );
        Ok(())
    }
}