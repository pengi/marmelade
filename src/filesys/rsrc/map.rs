use crate::serialization::{SerialRead, SerialReadStorage};
use crate::types::{
    PString,
    OSType
};

use super::types::{
    RsrcMapHeader,
    RsrcTypeRef,
    RsrcRef
};

#[derive(Debug)]
pub struct RsrcObj {
    pub id : i16,
    pub name : Option<PString>,
    pub attributes : u8,
    pub data_offset : u64
}

impl RsrcObj {
    fn read(rdr: &mut SerialReadStorage, maphdr: &RsrcMapHeader) -> std::io::Result<RsrcObj> {
        let refobj = RsrcRef::read(rdr)?;

        let name = if refobj.name_offset >= 0 {
            rdr.length_start(0); // So we can jump back
            rdr.seek(maphdr.name_list_offset as u64 + refobj.name_offset as u64);
            let name = Some(PString::read(rdr)?);
            rdr.length_end();
            name
        } else {
            None
        };

        Ok(RsrcObj{
            id: refobj.id,
            name: name,
            attributes: refobj.attributes,
            data_offset: refobj.data_offset as u64
        })
    }
}

#[derive(Debug)]
pub struct RsrcType {
    pub rsrc_type: OSType,
    pub rsrc: Vec<RsrcObj>
}

#[derive(Debug)]
pub struct RsrcMap {
    pub attributes: u16,
    pub types: Vec<RsrcType>
}

impl SerialRead for RsrcMap {
    fn read(rdr: &mut SerialReadStorage) -> std::io::Result<RsrcMap> {
        let maphdr = RsrcMapHeader::read(rdr)?;

        rdr.seek(maphdr.type_list_offset as u64);
        let count = u16::read(rdr)? + 1;

        let mut type_refs = Vec::with_capacity(count as usize);
        for _ in 0..count {
            type_refs.push(RsrcTypeRef::read(rdr)?);
        }

        let mut types = Vec::with_capacity(count as usize);
        for t in type_refs.into_iter() {
            rdr.seek(maphdr.type_list_offset as u64 + t.type_offset as u64);
            let mut rsrc = Vec::with_capacity(t.count as usize+1);
            for _ in 0..t.count+1 {
                rsrc.push(RsrcObj::read(rdr, &maphdr)?);
            }
            types.push(RsrcType{
                rsrc_type: t.rsrc_type,
                rsrc: rsrc
            });
        }

        Ok(RsrcMap{
            attributes: maphdr.attributes,
            types: types,
        })
    }
}

impl RsrcMap {
    pub fn open(&self, rsrc_type: OSType, id: i16) -> Option<&RsrcObj> {
        for typelist in self.types.iter() {
            if typelist.rsrc_type == rsrc_type {
                for obj in typelist.rsrc.iter() {
                    if obj.id == id {
                        return Some(&obj)
                    }
                }
            }
        }
        None
    }
}