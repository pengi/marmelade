mod types;
mod map;

use crate::serialization::{SerialAccess, SerialRead};

use types::{
    RsrcHeader
};

use map::RsrcMap;

#[derive(Debug)]
pub struct Rsrc {
    storage: Box<dyn SerialAccess>,
    header: RsrcHeader,
    map: RsrcMap
}

impl Rsrc {
    pub fn new(storage: Box<dyn SerialAccess>) -> std::io::Result<Rsrc> {
        let mut storage = storage;
        storage.seek(0)?;
        let mut rdr = storage.read(16)?;
        let header = RsrcHeader::read(&mut rdr)?;

        storage.seek(header.map_offset as u64)?;
        let mut rdr = storage.read(header.map_len as u64)?;
        let map = RsrcMap::read(&mut rdr)?;

        Ok(Rsrc{
            storage,
            header,
            map
        })
    }
}