use crate::serialread::{SerialReadStorage, SerialRead};
use chrono::NaiveDateTime;

#[derive(PartialEq)]
#[derive(PartialOrd)]
pub struct PString (Vec<u8>);

impl std::fmt::Debug for PString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from(String::from_utf8_lossy(&self.0));
        std::fmt::Debug::fmt(&s, f)
    }
}

impl std::fmt::Display for PString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from(String::from_utf8_lossy(&self.0));
        std::fmt::Display::fmt(&s, f)
    }
}

impl SerialRead for PString {
    fn read(rdr : &mut SerialReadStorage ) -> std::io::Result<PString> {
        let len : u8 = SerialRead::read(rdr)?;
        let mut data = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let val : u8 = SerialRead::read(rdr)?;
            data.push(val)
        }
        Ok(PString (
            data
        ))
    }
}

impl From<&str> for PString {
    fn from(s: &str) -> PString {
        PString(Vec::from(s))
    }
}

impl From<&PString> for String {
    fn from(s: &PString) -> String {
        String::from(String::from_utf8_lossy(&s.0[..]))
    }
}

#[derive(Debug)]
pub struct DateTime (NaiveDateTime);

impl SerialRead for DateTime {
    fn read(rdr:&mut SerialReadStorage) -> std::io::Result<DateTime> {
        let val : u32 = SerialRead::read(rdr)?;
        Ok( DateTime (NaiveDateTime::from_timestamp(val as i64 - 2082844800i64, 0)))
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(SerialRead)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct ExtDescriptor {
    pub xdrStABN: u16,    // first allocation block
    pub xdrNumABlks: i16, // number of allocation blocks
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(SerialRead)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct ExtDataRec(
    pub [ExtDescriptor; 3]
);
