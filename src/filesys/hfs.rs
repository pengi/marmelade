mod mdb;
mod volbitmap;
mod btree;
pub mod fileadaptor;

use std::io;
use std::cell::RefCell;

use fileadaptor::FileAccess;

use mdb::HfsMDB;
use mdb::ExtDataRec;
use volbitmap::HfsVolBitmap;
use btree::BTreeIter;

#[derive(Debug)]
pub struct HfsImage<'storage>
{
    storage: RefCell<&'storage mut dyn FileAccess>,
    mdb: HfsMDB,
    bitmap: HfsVolBitmap,
    start_of_alloc: u64,
    size: u64,
}

impl<'storage> HfsImage<'storage>
{
    pub fn from(storage: &mut dyn FileAccess) -> io::Result<HfsImage> {
        let size = storage.size()?;

        storage.seek(512*2)?;
        let mdb = HfsMDB::from(storage)?;

        storage.seek(512*(mdb.drVBMSt as u64))?;
        let bitmap = HfsVolBitmap::from(storage, mdb.drNmAlBlks)?;
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
        let _iter = BTreeIter::scan(self, &self.mdb.drCTExtRec)?;
        Ok(())
    }

    fn read_ext_rec(&self, rec: &ExtDataRec, start : u64, len : usize) -> io::Result<Vec<u8>> {
        let mut f = self.storage.borrow_mut();
        let range = &rec.0[0];
        let offset : u64 = self.start_of_alloc + range.xdrStABN as u64 * self.mdb.drAlBlkSiz as u64 + start;
        let range_len : usize = range.xdrNumABlks as usize *  self.mdb.drAlBlkSiz as usize;
        if range_len < start as usize + len {
            return Err(io::Error::new(io::ErrorKind::Other, "Record read out of range"));
        }
        f.seek(offset)?;
        Ok(f.read_vec(len)?)
    }
}
