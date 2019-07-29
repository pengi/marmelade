use super::{
    FileReader,
    FileReadable,
    common::{
        PString,
        DateTime,
        ExtDataRec
    }
};

#[derive(FileReadable)]
#[derive(Debug)]
pub struct Point (i16, i16);

#[derive(FileReadable)]
#[derive(Debug)]
pub struct Rect (i16, i16, i16, i16);

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct FInfo {
    fdType:     u32, // OSType;     {file type}
    fdCreator:  u32, // OSType;     {file creator}
    fdFlags:    u16, // Integer;    {Finder flags}
    fdLocation: Point, // Point;      {file's location in window}
    fdFldr:     u16, // Integer;    {directory that contains file}
}

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct FXInfo {
    fdIconID:      i16, // Integer;    {icon ID}
    fdUnused:      [i16; 3], // ARRAY[1..3] OF Integer; {unused but reserved 6 bytes}
    fdScript:      i8, // SignedByte; {script flag and code}
    fdXFlags:      i8, // SignedByte; {reserved}
    fdComment:     i16, // Integer;    {comment ID}
    fdPutAway:     u32, // LongInt;    {home directory ID}
}

// TODO: Reverse engineered
#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct DInfo {
    frRect:     Rect,  // Rect;    {folder's window rectangle}
    frFlags:    u16,   // Integer; {flags}
    frLocation: Point, // Point;   {folder's location in window}
    frView:     u16,   // Integer; {folder's view}
}

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct DXInfo {
    frScroll:      Point, // Point;      {scroll position}
    frOpenChain:   u32, // LongInt;    {directory ID chain of open folders}
    frScript:      i8, // SignedByte; {script flag and code}
    frXFlags:      u8, // SignedByte; {reserved}
    frComment:     i16, // Integer;    {comment ID}
    frPutAway:     u32, // LongInt;    {home directory ID}
}

#[derive(PartialOrd)]
#[derive(PartialEq)]
#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct CatKeyRec {
    pub ckrKeyLen : u8,
    pub ckrResrv1 : u8,
    pub ckrParID : u32,
    pub ckrCName : PString,
}

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct CdrDirRec {
   dirFlags:      u16, // Integer;    {directory flags}
   dirVal:        i16, // Integer;    {directory valence}
   dirDirID:      u32, // LongInt;    {directory ID}
   dirCrDat:      DateTime, // LongInt;    {date and time of creation}
   dirMdDat:      DateTime, // LongInt;    {date and time of last modification}
   dirBkDat:      DateTime, // LongInt;    {date and time of last backup}
   dirUsrInfo:    DInfo, // DInfo;      {Finder information}
   dirFndrInfo:   DXInfo, // DXInfo;     {additional Finder information}
   dirResrv:      [u32; 4] // ARRAY[1..4] OF LongInt {reserved}
}

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct CdrFilRec {
   filFlags:      u8, // SignedByte; {file flags}
   filTyp:        u8, // SignedByte; {file type}
   filUsrWds:     FInfo,// FInfo;      {Finder information}
   filFlNum:      u32, // LongInt;    {file ID}
   filStBlk:      u16, // Integer;    {first alloc. blk. of data fork}
   filLgLen:      u32, // LongInt;    {logical EOF of data fork}
   filPyLen:      u32, // LongInt;    {physical EOF of data fork}
   filRStBlk:     u16, // Integer;    {first alloc. blk. of resource fork}
   filRLgLen:     u32, // LongInt;    {logical EOF of resource fork}
   filRPyLen:     u32, // LongInt;    {physical EOF of resource fork}
   filCrDat:      DateTime, // LongInt;    {date and time of creation}
   filMdDat:      DateTime, // LongInt;    {date and time of last modification}
   filBkDat:      DateTime, // LongInt;    {date and time of last backup}
   filFndrInfo:   FXInfo, // FXInfo;     {additional Finder information}
   filClpSize:    u16, // Integer;    {file clump size}
   filExtRec:     ExtDataRec, // ExtDataRec; {first data fork extent record}
   filRExtRec:    ExtDataRec, // ExtDataRec; {first resource fork extent record}
   filResrv:      u32, // LongInt     {reserved}
}

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct CdrThdRec {
   thdResrv:      [u32; 2], // ARRAY[1..2] OF LongInt; {reserved}
   thdParID:      u32,      // LongInt;    {parent ID for this directory}
   thdCName:      PString,  // Str31;     {name of this directory}
}

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct CdrFThdRec {
   fthdResrv:     [u32; 2], // ARRAY[1..2] OF LongInt; {reserved}
   fthdParID:     u32,      // LongInt;    {parent ID for this file}
   fthdCName:     PString,  // Str31;      {name of this file}
}

#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub enum CatDataRec {
    CdrDirRec(CdrDirRec),
    CdrFilRec(CdrFilRec),
    CdrThdRec(CdrThdRec),
    CdrFThdRec(CdrFThdRec),
}

#[derive(Debug)]
#[derive(FileReadable)]
#[allow(non_snake_case)]
struct CatDataRecHeader {
   cdrType:       i8, // SignedByte; {record type}
   cdrResrv2:     i8, // SignedByte; {reserved}
}

impl FileReadable for CatDataRec {
    fn read(rdr : &mut FileReader) -> std::io::Result<CatDataRec> {
        let header = CatDataRecHeader::read(rdr)?;
        Ok(match header.cdrType {
            1 => CatDataRec::CdrDirRec(CdrDirRec::read(rdr)?),
            2 => CatDataRec::CdrFilRec(CdrFilRec::read(rdr)?),
            3 => CatDataRec::CdrThdRec(CdrThdRec::read(rdr)?),
            4 => CatDataRec::CdrFThdRec(CdrFThdRec::read(rdr)?),
            _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
        })
    }
}