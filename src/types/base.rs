use crate::serialization::{SerialReadStorage, SerialRead};
use chrono::NaiveDateTime;

#[derive(PartialEq)]
#[derive(SerialRead)]
pub struct OSType ([u8;4]);

impl std::fmt::Debug for OSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\'{}{}{}{}\'",
            self.0[0] as char,
            self.0[1] as char,
            self.0[2] as char,
            self.0[3] as char
        )
    }
}

impl std::fmt::Display for OSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\'{}{}{}{}\'",
            self.0[0] as char,
            self.0[1] as char,
            self.0[2] as char,
            self.0[3] as char
        )
    }
}

impl From<&[u8; 4]> for OSType {
    fn from(b: &[u8; 4]) -> OSType {
        OSType(b.clone())
    }
}

#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Clone)]
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
