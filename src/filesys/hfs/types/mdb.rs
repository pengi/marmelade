use crate::serialread::{SerialReadStorage, SerialRead};

use super::common::{
    PString,
    DateTime,
    ExtDataRec
};

#[derive(Debug)]
#[derive(SerialRead)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct MDB {
    pub drSigWord: i16,         //Integer,    // volume signature
    pub drCrDate: DateTime,     //LongInt,    // date and time of volume creation
    pub drLsMod: DateTime,      //LongInt,    // date and time of last modification
    pub drAtrb: i16,            //Integer,    // volume attributes
    pub drNmFls: i16,           //Integer,    // number of files in root directory
    pub drVBMSt: i16,           //Integer,    // first block of volume bitmap
    pub drAllocPtr: i16,        //Integer,    // start of next allocation search
    pub drNmAlBlks: u16,        //Integer,    // number of allocation blocks in volume
    pub drAlBlkSiz: i32,        //LongInt,    // size (in bytes) of allocation blocks
    pub drClpSiz: i32,          //LongInt,    // default clump size
    pub drAlBlSt: i16,          //Integer,    // first allocation block in volume
    pub drNxtCNID: i32,         //LongInt,    // next unused catalog node ID
    pub drFreeBks: u16,         //Integer,    // number of unused allocation blocks
    #[length_start(28)]
    pub drVN: PString,          //String[27], // volume name
    #[length_end()]
    pub drVolBkUp: DateTime,    //LongInt,    // date and time of last backup
    pub drVSeqNum: i16,         //Integer,    // volume backup sequence number
    pub drWrCnt: i32,           //LongInt,    // volume write count
    pub drXTClpSiz: i32,        //LongInt,    // clump size for extents overflow file
    pub drCTClpSiz: i32,        //LongInt,    // clump size for catalog file
    pub drNmRtDirs: i16,        //Integer,    // number of directories in root directory
    pub drFilCnt: i32,          //LongInt,    // number of files in volume
    pub drDirCnt: i32,          //LongInt,    // number of directories in volume
    pub drFndrInfo: [i32; 8],   //ARRAY[1..8] OF LongInt, // information used by the Finder
    pub drVCSize: i16,          //Integer,    // size (in blocks) of volume cache
    pub drVBMCSize: i16,        //Integer,    // size (in blocks) of volume bitmap cache
    pub drCtlCSize: i16,        //Integer,    // size (in blocks) of common volume cache
    pub drXTFlSize: i32,        //LongInt,    // size of extents overflow file
    pub drXTExtRec: ExtDataRec, //ExtDataRec, // extent record for extents overflow file
    pub drCTFlSize: i32,        //LongInt,    // size of catalog file
    pub drCTExtRec: ExtDataRec, //ExtDataRec, // extent record for catalog file
}