use std::io;
use super::fileadaptor::FileBlock;
use super::fileadaptor::FileBlockSeqReader;

#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct ExtDescriptor {
    pub xdrStABN: u16,                      // first allocation block
    pub xdrNumABlks: i16,                   // number of allocation blocks
}

#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct ExtDataRec (pub [ExtDescriptor; 3]);


#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct HfsMDB {
    pub drSigWord: i16,       //Integer,    // volume signature
    pub drCrDate: i32,        //LongInt,    // date and time of volume creation
    pub drLsMod: i32,         //LongInt,    // date and time of last modification
    pub drAtrb: i16,          //Integer,    // volume attributes
    pub drNmFls: i16,         //Integer,    // number of files in root directory
    pub drVBMSt: i16,         //Integer,    // first block of volume bitmap
    pub drAllocPtr: i16,      //Integer,    // start of next allocation search
    pub drNmAlBlks: u16,      //Integer,    // number of allocation blocks in volume
    pub drAlBlkSiz: i32,      //LongInt,    // size (in bytes) of allocation blocks
    pub drClpSiz: i32,        //LongInt,    // default clump size
    pub drAlBlSt: i16,        //Integer,    // first allocation block in volume
    pub drNxtCNID: i32,       //LongInt,    // next unused catalog node ID
    pub drFreeBks: u16,       //Integer,    // number of unused allocation blocks
    pub drVN: String,         //String[27], // volume name
    pub drVolBkUp: i32,       //LongInt,    // date and time of last backup
    pub drVSeqNum: i16,       //Integer,    // volume backup sequence number
    pub drWrCnt: i32,         //LongInt,    // volume write count
    pub drXTClpSiz: i32,      //LongInt,    // clump size for extents overflow file
    pub drCTClpSiz: i32,      //LongInt,    // clump size for catalog file
    pub drNmRtDirs: i16,      //Integer,    // number of directories in root directory
    pub drFilCnt: i32,        //LongInt,    // number of files in volume
    pub drDirCnt: i32,        //LongInt,    // number of directories in volume
    pub drFndrInfo: [i32; 8], //ARRAY[1..8] OF LongInt, // information used by the Finder
    pub drVCSize: i16,        //Integer,    // size (in blocks) of volume cache
    pub drVBMCSize: i16,      //Integer,    // size (in blocks) of volume bitmap cache
    pub drCtlCSize: i16,      //Integer,    // size (in blocks) of common volume cache
    pub drXTFlSize: i32,      //LongInt,    // size of extents overflow file
    pub drXTExtRec: ExtDataRec, //ExtDataRec, // extent record for extents overflow file
    pub drCTFlSize: i32,    //LongInt,    // size of catalog file
    pub drCTExtRec: ExtDataRec, //ExtDataRec, // extent record for catalog file
}

impl ExtDescriptor {
    #[allow(non_snake_case)] // This struct comes from old Mac structs
    fn from(rdr: &mut FileBlockSeqReader) -> ExtDescriptor {
        let xdrStABN = rdr.read_u16();
        let xdrNumABlks = rdr.read_i16();
        ExtDescriptor{xdrStABN,xdrNumABlks}
    }
}

impl ExtDataRec {
    fn from(rdr: &mut FileBlockSeqReader) -> ExtDataRec {
        ExtDataRec([
            ExtDescriptor::from(rdr),
            ExtDescriptor::from(rdr),
            ExtDescriptor::from(rdr),
        ])
    }
}

impl HfsMDB {
    #[allow(non_snake_case)] // This struct comes from old Mac structs
    pub fn from(block: &FileBlock) -> io::Result<HfsMDB> {
        let mut rdr    = FileBlockSeqReader::from(block, 0);
        let drSigWord  = rdr.read_i16();
        let drCrDate   = rdr.read_i32();
        let drLsMod    = rdr.read_i32();
        let drAtrb     = rdr.read_i16();
        let drNmFls    = rdr.read_i16();
        let drVBMSt    = rdr.read_i16();
        let drAllocPtr = rdr.read_i16();
        let drNmAlBlks = rdr.read_u16();
        let drAlBlkSiz = rdr.read_i32();
        let drClpSiz   = rdr.read_i32();
        let drAlBlSt   = rdr.read_i16();
        let drNxtCNID  = rdr.read_i32();
        let drFreeBks  = rdr.read_u16();
        let drVN       = rdr.read_pstr(27);
        let drVolBkUp  = rdr.read_i32();
        let drVSeqNum  = rdr.read_i16();
        let drWrCnt    = rdr.read_i32();
        let drXTClpSiz = rdr.read_i32();
        let drCTClpSiz = rdr.read_i32();
        let drNmRtDirs = rdr.read_i16();
        let drFilCnt   = rdr.read_i32();
        let drDirCnt   = rdr.read_i32();
        let drFndrInfo = [
            rdr.read_i32(),
            rdr.read_i32(),
            rdr.read_i32(),
            rdr.read_i32(),
            rdr.read_i32(),
            rdr.read_i32(),
            rdr.read_i32(),
            rdr.read_i32(),
        ];
        let drVCSize   = rdr.read_i16();
        let drVBMCSize = rdr.read_i16();
        let drCtlCSize = rdr.read_i16();
        let drXTFlSize = rdr.read_i32();
        let drXTExtRec = ExtDataRec::from(&mut rdr);
        let drCTFlSize = rdr.read_i32();
        let drCTExtRec = ExtDataRec::from(&mut rdr);

        if drSigWord != 0x4244 {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid drSigWord"));
        }

        if drAlBlkSiz % 512 != 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid drAlBlkSiz"));
        }

        Ok(HfsMDB {
            drSigWord,
            drCrDate,
            drLsMod,
            drAtrb,
            drNmFls,
            drVBMSt,
            drAllocPtr,
            drNmAlBlks,
            drAlBlkSiz,
            drClpSiz,
            drAlBlSt,
            drNxtCNID,
            drFreeBks,
            drVN,
            drVolBkUp,
            drVSeqNum,
            drWrCnt,
            drXTClpSiz,
            drCTClpSiz,
            drNmRtDirs,
            drFilCnt,
            drDirCnt,
            drFndrInfo,
            drVCSize,
            drVBMCSize,
            drCtlCSize,
            drXTFlSize,
            drXTExtRec,
            drCTFlSize,
            drCTExtRec,
        })
    }
}
