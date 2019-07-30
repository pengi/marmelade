use super::fileadaptor::FileAccess;
use super::types::FileReader;

use std::cell::RefCell;
use std::rc::Rc;

use std::convert::From;
use super::types::common::ExtDataRec;

#[derive(Debug)]
#[derive(Clone)]
pub struct BlockAccess {
    storage: Rc<RefCell<Box<FileAccess>>>,
    alblk_start: u64,
    alblk_size: u64
}

impl BlockAccess {
    pub fn new(storage: Box<FileAccess>, alblk_start: u64, alblk_size: u64) -> BlockAccess {
        BlockAccess {
            storage: Rc::new(RefCell::new(storage)),
            alblk_start: alblk_start * 512,
            alblk_size
        }
    }

    fn do_read_blk(&self, offset: u64, len: u64) -> std::io::Result<FileReader> {
        let mut storage = self.storage.borrow_mut();
        storage.seek(offset)?;
        Ok(FileReader::from(storage.read(len)?))
    }

    // pub fn read_blk(&self, blknum : u64) -> std::io::Result<FileReader> {
    //     self.do_read_blk(blknum * 512, 512)
    // }

    // pub fn read_alblk(&self, blknum : u64) -> std::io::Result<FileReader> {
    //     self.do_read_blk(self.alblk_start + blknum * self.alblk_size, self.alblk_size)
    // }

    pub fn read_extdatarec(&self, rec : &ExtDataRec, offset : u64, len : u64) -> std::io::Result<FileReader> {
        // TODO: Support multiple extents
        self.do_read_blk(
            self.alblk_start + (rec.0[0].xdrStABN as u64) * self.alblk_size + offset,
            len)
    }
}