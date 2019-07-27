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
    pub recno : i32,
    pub data : Vec<u8>
}

impl BTreeRecord for BTreeVecRecord {
    fn new(block : &FileBlock, recno : i32, offset : usize, len : usize) -> Self {
        BTreeVecRecord{ recno, data: block.read_vec(offset, len) }
    }
}

impl std::fmt::Debug for BTreeVecRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BTreeVecRecord recno={} len={}>", self.recno, self.data.len())?;
        Ok(())
    }
}



#[derive(Debug)]
#[derive(Clone)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct BTreeHeader {
    bthDepth:    i16, //  Integer;    {current depth of tree}
    bthRoot:     i32, //  LongInt;    {number of root node}
    bthNRecs:    i32, //  LongInt;    {number of leaf records in tree}
    bthFNode:    i32, //  LongInt;    {number of first leaf node}
    bthLNode:    i32, //  LongInt;    {number of last leaf node}
    bthNodeSize: i16, //  Integer;    {size of a node}
    bthKeyLen:   i16, //  Integer;    {maximum length of a key}
    bthNNodes:   i32, //  LongInt;    {total number of nodes in tree}
    bthFree:     i32, //  LongInt;    {number of free nodes}
}

impl BTreeRecord for Option<BTreeHeader> {
    fn new(block : &FileBlock, recno : i32, offset : usize, _len : usize) -> Self {
        if recno == 0 {
            let mut rdr = FileBlockSeqReader::from(&block, offset);
            Some(BTreeHeader {
                bthDepth: rdr.read_i16(),
                bthRoot: rdr.read_i32(),
                bthNRecs: rdr.read_i32(),
                bthFNode: rdr.read_i32(),
                bthLNode: rdr.read_i32(),
                bthNodeSize: rdr.read_i16(),
                bthKeyLen: rdr.read_i16(),
                bthNNodes: rdr.read_i32(),
                bthFree: rdr.read_i32(),
            })
        } else {
            None
        }
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
    rec: &'iter ExtDataRec,
    header: BTreeHeader
}

fn read_block<'iter, 'fs, Rec : BTreeRecord>(fs: &'iter HfsImage<'fs>, rec: &'iter ExtDataRec, num: usize) -> io::Result<BTreeBlock<Rec>> {
    Ok(BTreeBlock::from(fs.read_ext_rec(rec, num*512, 512)?))
}

impl<'iter, 'fs> BTree<'iter, 'fs> {
    pub fn from(fs: &'iter HfsImage<'fs>, rec: &'iter ExtDataRec) -> io::Result<BTree<'iter, 'fs>> {
        let hdr_block = read_block::<Option<BTreeHeader>>(fs, rec, 0)?;
        let header = hdr_block.recs[0].clone();
        if let Some(header) = header {
            Ok(BTree{fs, rec, header})
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Unknown header"))
        }
    }
}

impl<'iter, 'fs> std::fmt::Debug for BTree<'iter, 'fs> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BTree.header=")?;
        self.header.fmt(f)
    }
}