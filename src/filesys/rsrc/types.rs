use crate::serialization::{
    SerialRead,
    SerialReadStorage
};
use crate::types::OSType;

#[derive(Debug)]
#[derive(SerialRead)]
pub struct RsrcHeader {
    pub data_offset : i32,
    pub map_offset : i32,
    pub data_len : u32,
    pub map_len : u32
}

#[derive(Debug)]
pub struct RsrcData {
    pub len : u32,
    pub data : SerialReadStorage
}

impl SerialRead for RsrcData {
    fn read(rdr: &mut SerialReadStorage) -> std::io::Result<RsrcData> {
        let len = u32::read(rdr)?;
        let data = rdr.sub_reader(rdr.pos(), len as u64);
        Ok(RsrcData { len, data })
    }
}

#[derive(Debug)]
#[derive(SerialRead)]
pub struct RsrcMapHeader {
    #[pad(22)]
    pub attributes : u16,
    pub type_list_offset : i16,
    pub name_list_offset : i16
}

#[derive(Debug)]
#[derive(SerialRead)]
pub struct RsrcTypeRef {
    pub rsrc_type : OSType,
    pub count : u16,
    pub type_offset : u16
}

#[derive(Debug)]
pub struct RsrcRef {
    pub id : i16,
    pub name_offset : i16,
    pub attributes : u8,
    pub data_offset : i32
}

impl SerialRead for RsrcRef {
    fn read(rdr: &mut SerialReadStorage) -> std::io::Result<RsrcRef> {
        let id = rdr.read_i16()?;
        let name_offset = rdr.read_i16()?;
        let attributes = rdr.read_u8()?;
        let data_offset = rdr.read_u24()? as i32;
        rdr.pad(4);
        Ok(RsrcRef {id, name_offset, attributes, data_offset})
    }
}