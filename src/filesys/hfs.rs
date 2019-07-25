mod mdb;
mod volbitmap;
pub mod fileadaptor;

use std::io;
use std::cell::RefCell;

use mdb::HfsMDB;
use mdb::ExtDataRec;
use volbitmap::HfsVolBitmap;
use fileadaptor::FileAccess;

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


        let img = HfsImage {
            storage: RefCell::new(storage),
            mdb,
            bitmap,
            start_of_alloc,
            size
            };

        println!("drXTExt: {:#?}", img.read_ext_rec(&img.mdb.drXTExtRec, 0, 64)?);
        println!("drCTExt: {:#?}", img.read_ext_rec(&img.mdb.drCTExtRec, 0, 64)?);


        Ok(img)
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
