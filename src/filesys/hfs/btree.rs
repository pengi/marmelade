use super::mdb::ExtDataRec;
use super::HfsImage;
use std::io;

use super::fileadaptor::{FileBlock, FileBlockSeqReader};

#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct NodeDescriptor {
    pub ndFLink: i32,  //     LongInt;       {forward link}
    pub ndBLink: i32,  //     LongInt;       {backward link}
    pub ndType: i8,    //     SignedByte;    {node type}
    pub ndNHeight: i8, //     SignedByte;    {node level}
    pub ndNRecs: i16,  //     Integer;       {number of records in node}
}

#[derive(Debug)]
pub struct BTreeBlock {
    pub block: FileBlock,
    pub nd: NodeDescriptor,
}

impl NodeDescriptor {
    pub fn from(block: &FileBlock) -> NodeDescriptor {
        let mut rdr = FileBlockSeqReader::from(&block, 0);
        NodeDescriptor {
            ndFLink: rdr.read_i32(),
            ndBLink: rdr.read_i32(),
            ndType: rdr.read_i8(),
            ndNHeight: rdr.read_i8(),
            ndNRecs: rdr.read_i16(),
        }
    }
}

impl BTreeBlock {
    pub fn from(block: FileBlock) -> BTreeBlock {
        let nd = NodeDescriptor::from(&block);
        BTreeBlock { block, nd }
    }
}

pub struct BTree<'iter, 'fs> {
    fs: &'iter HfsImage<'fs>,
    rec: &'iter ExtDataRec
}

impl<'iter, 'fs> BTree<'iter, 'fs> {
    pub fn read_block(&self, num: usize) -> io::Result<BTreeBlock> {
        Ok(BTreeBlock::from(self.fs.read_ext_rec(self.rec, num*512, 512)?))
    }

    pub fn scan(fs: &'iter HfsImage<'fs>, rec: &'iter ExtDataRec) -> BTree<'iter, 'fs> {
        BTree{fs, rec}
    }
}
