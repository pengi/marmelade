use crate::serialization::{SerialReadStorage, SerialRead};
use chrono::NaiveDateTime;

use crate::cpu::{CPU, Stackable};

#[derive(PartialEq)]
#[derive(SerialRead)]
pub struct OSType (pub [u8;4]);

impl OSType {
    pub fn as_u32(&self) -> u32 {
        let [a,b,c,d] = self.0;
        ((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | (d as u32)
    }
}

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

impl From<&[u8]> for OSType {
    fn from(b: &[u8]) -> OSType {
        assert_eq!(b.len(), 4);
        OSType([b[0], b[1], b[2], b[3]])
    }
}

impl From<u32> for OSType {
    fn from(b: u32) -> OSType {
        OSType([
            ((b>>24) & 0xff) as u8,
            ((b>>16) & 0xff) as u8,
            ((b>>8) & 0xff) as u8,
            ((b>>0) & 0xff) as u8,
        ])
    }
}

impl From<OSType> for u32 {
    fn from(t: OSType) -> u32 {
        t.as_u32()
    }
}

impl Stackable for OSType {
    fn stack_push(&self, cpu: &mut CPU) {
        self.as_u32().stack_push(cpu);
    }
    fn stack_pop(cpu: &mut CPU) -> Self {
        OSType::from(u32::stack_pop(cpu))
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
