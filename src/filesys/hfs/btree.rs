use super::HfsImage;
use super::mdb::ExtDataRec;
use std::io;

#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
struct NodeDescriptor {
   pub ndFLink:   i32, //     LongInt;       {forward link}
   pub ndBLink:   i32, //     LongInt;       {backward link}
   pub ndType:    i8,  //     SignedByte;    {node type}
   pub ndNHeight: i8,  //     SignedByte;    {node level}
   pub ndNRecs:   i16, //     Integer;       {number of records in node}
}

pub struct BTreeIter {

}

impl BTreeIter {
    pub fn scan(fs : &HfsImage, rec : &ExtDataRec) -> io::Result<BTreeIter> {
        println!("Listing files");
        println!("drCTExt: {:?}", fs.read_ext_rec(rec, 0, 64)?);
        Ok(BTreeIter{})
    }
}