use super::super::block::{
    FileReader
};

use super::FileReadable;

// pub struct PString (Vec<u8>);

// impl std::fmt::Debug for PString {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let s = String::from(String::from_utf8_lossy(&self.0));
//         write!(f, "{:?}", s)
//     }
// }

// impl FileReadable for PString {
//     fn read(rdr : &mut FileReader ) -> PString {
        
//     }
// }

#[derive(Debug)]
#[derive(FileReadable)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct ExtDescriptor {
    pub xdrStABN: u16,    // first allocation block
    pub xdrNumABlks: i16, // number of allocation blocks
}

#[derive(Debug)]
#[derive(FileReadable)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct ExtDataRec(
    pub [ExtDescriptor; 3]
);
