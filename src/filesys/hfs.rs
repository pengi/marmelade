mod mdb;
mod volbitmap;
pub mod fileadaptor;

use std::io;
use std::cell::RefCell;

use mdb::HfsMDB;
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

        println!("drXTExt: {:#?}", img.get_alloc_block(img.mdb.drXTExtRec.0[0].xdrStABN)?);
        println!("drCTExt: {:#?}", img.get_alloc_block(img.mdb.drCTExtRec.0[0].xdrStABN)?);


        Ok(img)
    }

    fn get_alloc_block(&self, blocknum : u16) -> io::Result<Vec<u8>> {
        let mut f = self.storage.borrow_mut();
        f.seek(blocknum as u64 * self.mdb.drAlBlkSiz as u64 + self.start_of_alloc)?;
        Ok(f.read_vec(self.mdb.drAlBlkSiz as usize)?)
    }
}
