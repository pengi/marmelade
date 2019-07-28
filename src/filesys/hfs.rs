mod types;
mod block;
pub mod fileadaptor;

use std::io;
use std::cell::RefCell;

use fileadaptor::FileAccess;
use block::FileReader;
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
        let mut mdb_block : FileReader = FileReader::from(storage.read(512)?);
        let mdb = types::mdb::MDB::read(&mut mdb_block);
        
        Ok(HfsImage {
            storage: RefCell::new(storage),
            mdb
            })
    }
}
