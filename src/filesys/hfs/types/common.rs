use super::{FileReader, FileReadable};
use chrono::NaiveDateTime;

pub struct PString (Vec<u8>);

impl std::fmt::Debug for PString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from(String::from_utf8_lossy(&self.0));
        write!(f, "{:?}", s)
    }
}

impl FileReadable for PString {
    fn read(rdr : &mut FileReader ) -> PString {
        let len : u8 = FileReadable::read(rdr);
        let mut data = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let val : u8 = FileReadable::read(rdr);
            data.push(val)
        }
        PString (
            data
        )
    }
}

#[derive(Debug)]
pub struct DateTime (NaiveDateTime);

impl FileReadable for DateTime {
    fn read(rdr:&mut FileReader) -> DateTime {
        let val : u32 = FileReadable::read(rdr);
        DateTime (NaiveDateTime::from_timestamp(val as i64 - 2082844800i64, 0))
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(FileReadable)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct ExtDescriptor {
    pub xdrStABN: u16,    // first allocation block
    pub xdrNumABlks: i16, // number of allocation blocks
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(FileReadable)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct ExtDataRec(
    pub [ExtDescriptor; 3]
);
