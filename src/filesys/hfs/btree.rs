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

pub trait BTreeRecord {
    fn new(block : &FileBlock, recno : i32, offset : usize, len : usize) -> Self;
}

pub struct BTreeVecRecord {
    pub data : Vec<u8>
}

impl BTreeRecord for BTreeVecRecord {
    fn new(block : &FileBlock, _recno : i32, offset : usize, len : usize) -> Self {
        BTreeVecRecord{ data: block.read_vec(offset, len) }
    }
}

impl std::fmt::Debug for BTreeVecRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BTreeVecRecord len={}>", self.data.len())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct BTreeBlock<Rec : BTreeRecord> {
    pub block: FileBlock,
    pub nd: NodeDescriptor,
    pub recs : Vec<Rec>
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

impl<Rec : BTreeRecord> BTreeBlock<Rec> {
    pub fn from(block: FileBlock) -> BTreeBlock<Rec> {
        let nd = NodeDescriptor::from(&block);
        let mut recs : Vec<Rec> = Vec::with_capacity(nd.ndNRecs as usize);
        for i in 0..nd.ndNRecs {
            let idx_start = block.read_u16((512-2-i*2) as usize);
            let idx_end = block.read_u16((512-2-i*2-2) as usize);
            recs.push(Rec::new(&block, i.into(), idx_start.into(), (idx_end-idx_start).into()));
        };
        BTreeBlock { block, nd, recs }
    }
}

pub struct BTree<'iter, 'fs> {
    fs: &'iter HfsImage<'fs>,
    rec: &'iter ExtDataRec
}

impl<'iter, 'fs> BTree<'iter, 'fs> {
    pub fn read_block<Rec : BTreeRecord>(&self, num: usize) -> io::Result<BTreeBlock<Rec>> {
        Ok(BTreeBlock::from(self.fs.read_ext_rec(self.rec, num*512, 512)?))
    }

    pub fn from(fs: &'iter HfsImage<'fs>, rec: &'iter ExtDataRec) -> BTree<'iter, 'fs> {
        BTree{fs, rec}
    }
}
