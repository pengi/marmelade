mod types;
mod block;
pub mod fileadaptor;

use std::io;
use std::cell::RefCell;

use fileadaptor::FileAccess;
use block::{
    FileReader
};
use types::FileReadable;

#[derive(Debug)]
pub struct HfsImage<'storage>
{
    storage: RefCell<&'storage mut dyn FileAccess>,
    mdb: types::mdb::MDB,
}

impl<'storage> HfsImage<'storage>
{
    pub fn from(storage: &mut dyn FileAccess) -> io::Result<HfsImage> {
        // let size = storage.size()?;
        storage.seek(512*2)?;
        let mut mdb_block : FileReader = storage.read(512)?.to_reader();
        let mdb = types::mdb::MDB::read(&mut mdb_block);
        
        Ok(HfsImage {
            storage: RefCell::new(storage),
            mdb
            })
    }

    // pub fn list_files(&self) -> io::Result<()> {
    //     let xtbtree = BTree::from(self, &self.mdb.drXTExtRec)?;
    //     let ctbtree = BTree::from(self, &self.mdb.drCTExtRec)?;
    //     println!("{:#?}", xtbtree);
    //     xtbtree.scan()?;
    //     println!("{:#?}", ctbtree);
    //     ctbtree.scan()?;
    //     Ok(())
    // }

    // fn read_ext_rec(&self, rec: &ExtDataRec, start : usize, len : usize) -> io::Result<FileBlock> {
    //     let mut f = self.storage.borrow_mut();
    //     let range = &rec.0[0];
    //     let offset : u64 = self.start_of_alloc + range.xdrStABN as u64 * self.mdb.drAlBlkSiz as u64 + start as u64;
    //     let range_len : usize = range.xdrNumABlks as usize *  self.mdb.drAlBlkSiz as usize;
    //     if range_len < start as usize + len {
    //         return Err(io::Error::new(io::ErrorKind::Other, "Record read out of range"));
    //     }
    //     f.seek(offset)?;
    //     Ok(f.read_vec(len)?)
    // }
}
