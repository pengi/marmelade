mod mdb;
mod volbitmap;

use std::io;
use std::io::SeekFrom;
use std::cell::RefCell;

use mdb::HfsMDB;
use volbitmap::HfsVolBitmap;

#[derive(Debug)]
pub struct HfsImage<'storage, T>
where
    T: io::Read + io::Seek,
{
    storage: RefCell<&'storage mut T>,
    mdb: HfsMDB,
    bitmap: HfsVolBitmap,
    start_of_alloc: u64,
    size: u64,
}

impl<'storage, T> HfsImage<'storage, T>
where
    T: io::Read + io::Seek,
{
    pub fn from(storage: &'storage mut T) -> io::Result<HfsImage<T>> {
        let size = storage.seek(SeekFrom::End(0))?;

        storage.seek(SeekFrom::Start(512*2))?;
        let mdb = HfsMDB::from(storage)?;

        storage.seek(SeekFrom::Start(512*(mdb.drVBMSt as u64)))?;
        let bitmap = HfsVolBitmap::from(storage, mdb.drNmAlBlks)?;
        let start_of_alloc = storage.seek(SeekFrom::Current(0))?;


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
        let mut block = [0u8; 512];
        let mut f = self.storage.borrow_mut();
        f.seek(SeekFrom::Start(blocknum as u64 * self.mdb.drAlBlkSiz as u64 + self.start_of_alloc))?;
        f.read_exact(&mut block)?;
        Ok(block.to_vec())
    }
}
