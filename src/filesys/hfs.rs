mod mdb;
mod volbitmap;
mod btree;
pub mod fileadaptor;

use std::io;
use std::cell::RefCell;

use fileadaptor::FileAccess;
use fileadaptor::FileBlock;

use mdb::HfsMDB;
use mdb::ExtDataRec;
use volbitmap::HfsVolBitmap;
use btree::{BTree, BTreeVecRecord};

#[derive(Debug)]
pub struct HfsImage<'storage>
{
    storage: RefCell<&'storage mut dyn FileAccess>,
    mdb: HfsMDB,
    bitmap: HfsVolBitmap,
    start_of_alloc: u64,
    size: u64,
}

fn block_size_for_bits(bits : usize) -> usize {
    let bits_per_block = 512*8;
    let num_vbm_blocks = (bits + bits_per_block - 1) / bits_per_block;
    num_vbm_blocks * 512
}

impl<'storage> HfsImage<'storage>
{
    pub fn from(storage: &mut dyn FileAccess) -> io::Result<HfsImage> {
        let size = storage.size()?;

        storage.seek(512*2)?;
        let mdb = HfsMDB::from(&storage.read_vec(512)?)?;

        storage.seek(512*(mdb.drVBMSt as u64))?;
        let bitmap = HfsVolBitmap::from(storage.read_vec(block_size_for_bits(mdb.drNmAlBlks as usize))?);
        let start_of_alloc = storage.pos()?;

        Ok(HfsImage {
            storage: RefCell::new(storage),
            mdb,
            bitmap,
            start_of_alloc,
            size
            })
    }

    pub fn list_files(&self) -> io::Result<()> {
        let iter = BTree::from(self, &self.mdb.drCTExtRec);
        println!("{:#?}", iter.read_block::<BTreeVecRecord>(0)?);
        println!("{:#?}", iter.read_block::<BTreeVecRecord>(1)?);
        println!("{:#?}", iter.read_block::<BTreeVecRecord>(2)?);
        println!("{:#?}", iter.read_block::<BTreeVecRecord>(3)?);
        Ok(())
    }

    fn read_ext_rec(&self, rec: &ExtDataRec, start : usize, len : usize) -> io::Result<FileBlock> {
        let mut f = self.storage.borrow_mut();
        let range = &rec.0[0];
        let offset : u64 = self.start_of_alloc + range.xdrStABN as u64 * self.mdb.drAlBlkSiz as u64 + start as u64;
        let range_len : usize = range.xdrNumABlks as usize *  self.mdb.drAlBlkSiz as usize;
        if range_len < start as usize + len {
            return Err(io::Error::new(io::ErrorKind::Other, "Record read out of range"));
        }
        f.seek(offset)?;
        Ok(f.read_vec(len)?)
    }
}
