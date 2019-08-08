mod types;
mod map;

use crate::serialization::{SerialAccess, SerialRead, SerialReadStorage};

use types::{
    RsrcHeader
};
use crate::types::OSType;

use map::RsrcMap;

#[derive(Debug)]
pub struct Rsrc {
    storage: Box<dyn SerialAccess>,
    header: RsrcHeader,
    map: RsrcMap
}

impl Rsrc {
    pub fn new(storage: Box<dyn SerialAccess>) -> std::io::Result<Rsrc> {
        let mut rdr = storage.read(0, 16)?;
        let header = RsrcHeader::read(&mut rdr)?;

        let mut rdr = storage.read(header.map_offset as u64, header.map_len as u64)?;
        let map = RsrcMap::read(&mut rdr)?;

        Ok(Rsrc{
            storage,
            header,
            map
        })
    }

    pub fn open(&mut self, rsrc_type: OSType, id: i16) -> std::io::Result<SerialReadStorage> {
        let rsrcref = self
            .map.open(rsrc_type, id)
            .ok_or(std::io::Error::from(
                    std::io::ErrorKind::NotFound
                ))?;

        let mut size_rdr = self.storage.read(self.header.data_offset as u64 + rsrcref.data_offset, 4)?;
        let size = u32::read(&mut size_rdr)?;

        self.storage.read(self.header.data_offset as u64 + rsrcref.data_offset+4, size as u64)
    }
}