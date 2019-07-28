mod types;
mod blockaccess;
pub mod fileadaptor;

use std::io;

use fileadaptor::FileAccess;
use types::{
    FileReader,
    FileReadable,
    mdb::MDB
    };
use blockaccess::BlockAccess;


use types::btree::NodeDescriptor;

#[derive(Debug)]
pub struct HfsImage<'storage>
{
    storage: BlockAccess<'storage>,
    mdb: MDB,
}

impl<'storage> HfsImage<'storage>
{
    pub fn from(storage: &mut dyn FileAccess) -> io::Result<HfsImage> {
        // let size = storage.size()?;

        // Bootstrap with getting header, to get block size information
        storage.seek(2*512)?;
        let mut mdb_block : FileReader = FileReader::from(storage.read(512)?);
        let mdb = MDB::read(&mut mdb_block);

        // Set up block access
        let storage = BlockAccess::new(storage, mdb.drAlBlSt as u64, mdb.drAlBlkSiz as u64);

        let mut hdrblock = storage.read_extdatarec(&mdb.drCTExtRec, 0, 512)?;
        let nd = NodeDescriptor::read(&mut hdrblock);
        println!("{:?}", nd);

        Ok(HfsImage {storage, mdb})
    }
}
