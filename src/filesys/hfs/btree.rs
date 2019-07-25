use super::HfsImage;
use super::mdb::ExtDataRec;
use std::io;

pub struct BTreeIter {

}

impl BTreeIter {
    pub fn scan(fs : &HfsImage, rec : &ExtDataRec) -> io::Result<BTreeIter> {
        println!("Listing files");
        println!("drCTExt: {:#?}", fs.read_ext_rec(rec, 0, 64)?);
        Ok(BTreeIter{})
    }
}