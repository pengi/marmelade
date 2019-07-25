use std::io;
use super::fileadaptor::FileAccess;

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

pub fn readstr(f : &mut FileAccess, len : usize) -> io::Result<String> {
    // input len is the number of bytes, excluding length prefix. Include length
    let len = len + 1;

    let bufv = f.read_vec(len)?;

    let name = &bufv[1..(1+bufv[0] as usize)];
    let name = String::from_utf8_lossy(name);
    let name = String::from(name);
    println!("Buf: {:?}", &name);
    Ok(name)
}

impl ExtDescriptor {
    #[allow(non_snake_case)] // This struct comes from old Mac structs
    fn from(file: &mut FileAccess) -> io::Result<ExtDescriptor> {
        let xdrStABN = file.read_u16()?;
        let xdrNumABlks = file.read_i16()?;
        Ok(ExtDescriptor{xdrStABN,xdrNumABlks})
    }
}

impl ExtDataRec {
    fn from(file: &mut FileAccess) -> io::Result<ExtDataRec> {
        Ok(ExtDataRec([
            ExtDescriptor::from(file)?,
            ExtDescriptor::from(file)?,
            ExtDescriptor::from(file)?,
        ]))
    }
}

impl HfsMDB {
    #[allow(non_snake_case)] // This struct comes from old Mac structs
    pub fn from(file: &mut FileAccess) -> io::Result<HfsMDB> {
        let drSigWord = file.read_i16()?;
        let drCrDate = file.read_i32()?;
        let drLsMod = file.read_i32()?;
        let drAtrb = file.read_i16()?;
        let drNmFls = file.read_i16()?;
        let drVBMSt = file.read_i16()?;
        let drAllocPtr = file.read_i16()?;
        let drNmAlBlks = file.read_u16()?;
        let drAlBlkSiz = file.read_i32()?;
        let drClpSiz = file.read_i32()?;
        let drAlBlSt = file.read_i16()?;
        let drNxtCNID = file.read_i32()?;
        let drFreeBks = file.read_u16()?;
        let drVN = readstr(file, 27)?;
        let drVolBkUp = file.read_i32()?;
        let drVSeqNum = file.read_i16()?;
        let drWrCnt = file.read_i32()?;
        let drXTClpSiz = file.read_i32()?;
        let drCTClpSiz = file.read_i32()?;
        let drNmRtDirs = file.read_i16()?;
        let drFilCnt = file.read_i32()?;
        let drDirCnt = file.read_i32()?;
        let mut drFndrInfo: [i32; 8] = [0; 8];
        for el in &mut drFndrInfo {
            *el = file.read_i32()?;
        }
        let drVCSize = file.read_i16()?;
        let drVBMCSize = file.read_i16()?;
        let drCtlCSize = file.read_i16()?;
        let drXTFlSize = file.read_i32()?;
        let drXTExtRec = ExtDataRec::from(file)?;
        let drCTFlSize = file.read_i32()?;
        let drCTExtRec = ExtDataRec::from(file)?;

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
