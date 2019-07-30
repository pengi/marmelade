mod types;
mod blockaccess;
mod btree;
mod catalog;
pub mod fileadaptor;

use std::io;

use fileadaptor::FileAccess;
use types::{
    FileReader,
    FileReadable,
    mdb::MDB,
    catalog::CatDataRec
    };
use blockaccess::BlockAccess;

use catalog::Catalog;

#[derive(Debug)]
pub struct HfsImage
{
    storage: BlockAccess,
    mdb: MDB,
    pub catalog: Catalog
}

impl HfsImage
{
    pub fn from(storage: Box<FileAccess>) -> io::Result<HfsImage> {
        let mut storage = storage;
        // let size = storage.size()?;

        // Bootstrap with getting header, to get block size information
        storage.seek(2*512)?;
        let mut mdb_block : FileReader = FileReader::from(storage.read(512)?);
        let mdb = MDB::read(&mut mdb_block)?;

        // Set up block access
        let storage = BlockAccess::new(storage, mdb.drAlBlSt as u64, mdb.drAlBlkSiz as u64);

        let catalog = Catalog::new(&storage, &mdb.drCTExtRec)?;

        Ok(HfsImage {storage, mdb, catalog})
    }

    pub fn list_recursive(&self, dir : u32, indent: i32) {
        let indstr = String::from("  ").repeat(indent as usize);
        for (key, data) in self.catalog.dir(dir) {
            match data {
                CatDataRec::CdrDirRec(d) => {
                    println!("{}D {}: {}", indstr, d.dirDirID, key.ckrCName);
                    self.list_recursive(d.dirDirID, indent+1);
                }
                CatDataRec::CdrFilRec(f) => {
                    println!("{}F {}: {}", indstr, f.filFlNum, key.ckrCName);
                },
                CatDataRec::CdrThdRec(thd) | CatDataRec::CdrFThdRec(thd) => {
                    println!("{}T {}: {}", indstr, thd.thdParID, thd.thdCName);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct File {

}

#[derive(Debug)]
pub struct FileIter {

}

impl std::iter::Iterator for FileIter {
    type Item = File;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}