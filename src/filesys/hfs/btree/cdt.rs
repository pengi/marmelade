use super::super::fileadaptor::{FileBlock, FileBlockSeqReader};
use super::{BTreeHeader, BTreeRecord, NodeDescriptor};

#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub enum BTreeCDTRecord {
    cdrDirRec {
        dirFlags: u16, // Integer;    {directory flags}
        dirVal: i16,   // Integer;    {directory valence}
        dirDirID: i32, // LongInt;    {directory ID}
        dirCrDat: u32, // LongInt;    {date and time of creation}
        dirMdDat: u32, // LongInt;    {date and time of last modification}
        dirBkDat: u32, // LongInt;    {date and time of last backup}
                       //dirUsrInfo:    DInfo, // DInfo;      {Finder information}
                       //dirFndrInfo:   DXInfo,// DXInfo;     {additional Finder information}
                       //dirResrv:      // ARRAY[1..4] OF LongInt
    },
    cdrFilRec {
        // filFlags:      u8, // SignedByte; {file flags}
        // filTyp:        u8, // SignedByte; {file type}
        // filUsrWds:     FInfo, // FInfo;      {Finder information}
        // filFlNum:      i32, // LongInt;    {file ID}
        // filStBlk:      u16, // Integer;    {first alloc. blk. of data fork}
        // filLgLen:      u32, // LongInt;    {logical EOF of data fork}
        // filPyLen:      u32, // LongInt;    {physical EOF of data fork}
        // filRStBlk:     u16, // Integer;    {first alloc. blk. of resource fork}
        // filRLgLen:     u32, // LongInt;    {logical EOF of resource fork}
        // filRPyLen:     u32, // LongInt;    {physical EOF of resource fork}
        // filCrDat:      u32, // LongInt;    {date and time of creation}
        // filMdDat:      u32, // LongInt;    {date and time of last modification}
        // filBkDat:      u32, // LongInt;    {date and time of last backup}
        // filFndrInfo:   FXInfo, // FXInfo;     {additional Finder information}
        // filClpSize:    u16, // Integer;    {file clump size}
        // filExtRec:     // ExtDataRec; {first data fork extent record}
        // filRExtRec:    // ExtDataRec; {first resource fork extent record}
    },
    cdrThdRec {},
    cdrFThdRec {},
}

fn pad_to_wordlen(len: usize, wordlen: usize) -> usize {
    len + ((wordlen - 1) ^ ((len + wordlen - 1) & (wordlen - 1)))
}

#[cfg(test)]
mod tests {
    use super::pad_to_wordlen;
    #[test]
    fn pad_to_wordlen_2() {
        assert_eq!(0, pad_to_wordlen(0, 2));
        assert_eq!(2, pad_to_wordlen(1, 2));
        assert_eq!(2, pad_to_wordlen(2, 2));
        assert_eq!(4, pad_to_wordlen(3, 2));
        assert_eq!(4, pad_to_wordlen(4, 2));
        assert_eq!(6, pad_to_wordlen(5, 2));
        assert_eq!(6, pad_to_wordlen(6, 2));
        assert_eq!(8, pad_to_wordlen(7, 2));
        assert_eq!(8, pad_to_wordlen(8, 2));
    }
    #[test]
    fn pad_to_wordlen_4() {
        assert_eq!(0, pad_to_wordlen(0, 4));
        assert_eq!(4, pad_to_wordlen(1, 4));
        assert_eq!(4, pad_to_wordlen(2, 4));
        assert_eq!(4, pad_to_wordlen(3, 4));
        assert_eq!(4, pad_to_wordlen(4, 4));
        assert_eq!(8, pad_to_wordlen(5, 4));
        assert_eq!(8, pad_to_wordlen(6, 4));
        assert_eq!(8, pad_to_wordlen(7, 4));
        assert_eq!(8, pad_to_wordlen(8, 4));
    }
}

impl BTreeRecord for BTreeCDTRecord {
    fn new(
        _bth: &BTreeHeader,
        _nd: &NodeDescriptor,
        block: &FileBlock,
        _recno: i32,
        offset: usize,
        len: usize,
    ) -> Self {
        let mut rdr = FileBlockSeqReader::from(&block, offset);

        let keylen = rdr.read_u8() as usize;
        rdr.read_u8(); // Pad
        let dirid = rdr.read_u32();
        let name = rdr.read_pstr(31);

        print!("Parent: {:3} \"{:31}\"", dirid, name);

        // Increase keylen to include padding
        let dataoffset = pad_to_wordlen(offset + keylen + 1, 2);
        let mut rdr = FileBlockSeqReader::from(&block, dataoffset);

        let rectype = rdr.read_u8();

        for _ in dataoffset + 1..offset + len {
            print!(" {:02X}", rdr.read_u8());
        }
        println!("");

        match rectype {
            1 => BTreeCDTRecord::cdrDirRec {
                dirFlags: rdr.read_u16(),
                dirVal: rdr.read_i16(),
                dirDirID: rdr.read_i32(),
                dirCrDat: rdr.read_u32(),
                dirMdDat: rdr.read_u32(),
                dirBkDat: rdr.read_u32(),
            },
            2 => BTreeCDTRecord::cdrFilRec {},
            3 => BTreeCDTRecord::cdrThdRec {},
            4 => BTreeCDTRecord::cdrFThdRec {},
            _ => panic!("Should not happen {}", rectype),
        }
    }
}
