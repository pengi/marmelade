use super::{
    FileReader,
    FileReadable,
    common::{
        PString,
        DateTime,
        ExtDataRec
    }
};

#[derive(PartialEq)]
#[derive(FileReadable)]
pub struct OSType ([u8;4]);

impl std::fmt::Debug for OSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\'{}{}{}{}\'",
            self.0[0] as char,
            self.0[1] as char,
            self.0[2] as char,
            self.0[3] as char
        )
    }
}

impl std::fmt::Display for OSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\'{}{}{}{}\'",
            self.0[0] as char,
            self.0[1] as char,
            self.0[2] as char,
            self.0[3] as char
        )
    }
}

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct Point {
   v: i16, // INTEGER:     {vertical coordinate}
   h: i16  // INTEGER;     {horizontal  coordinate}
}

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct Rect {
   topLeft: Point,
   botRight: Point,
}

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct FInfo {
    fdType:     OSType, // OSType;     {file type}
    fdCreator:  OSType, // OSType;     {file creator}
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
   pub dirFlags:      u16, // Integer;    {directory flags}
   pub dirVal:        i16, // Integer;    {directory valence}
   pub dirDirID:      u32, // LongInt;    {directory ID}
   pub dirCrDat:      DateTime, // LongInt;    {date and time of creation}
   pub dirMdDat:      DateTime, // LongInt;    {date and time of last modification}
   pub dirBkDat:      DateTime, // LongInt;    {date and time of last backup}
   pub dirUsrInfo:    DInfo, // DInfo;      {Finder information}
   pub dirFndrInfo:   DXInfo, // DXInfo;     {additional Finder information}
   pub dirResrv:      [u32; 4] // ARRAY[1..4] OF LongInt {reserved}
}

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct CdrFilRec {
   pub filFlags:      u8, // SignedByte; {file flags}
   pub filTyp:        u8, // SignedByte; {file type}
   pub filUsrWds:     FInfo,// FInfo;      {Finder information}
   pub filFlNum:      u32, // LongInt;    {file ID}
   pub filStBlk:      u16, // Integer;    {first alloc. blk. of data fork}
   pub filLgLen:      u32, // LongInt;    {logical EOF of data fork}
   pub filPyLen:      u32, // LongInt;    {physical EOF of data fork}
   pub filRStBlk:     u16, // Integer;    {first alloc. blk. of resource fork}
   pub filRLgLen:     u32, // LongInt;    {logical EOF of resource fork}
   pub filRPyLen:     u32, // LongInt;    {physical EOF of resource fork}
   pub filCrDat:      DateTime, // LongInt;    {date and time of creation}
   pub filMdDat:      DateTime, // LongInt;    {date and time of last modification}
   pub filBkDat:      DateTime, // LongInt;    {date and time of last backup}
   pub filFndrInfo:   FXInfo, // FXInfo;     {additional Finder information}
   pub filClpSize:    u16, // Integer;    {file clump size}
   pub filExtRec:     ExtDataRec, // ExtDataRec; {first data fork extent record}
   pub filRExtRec:    ExtDataRec, // ExtDataRec; {first resource fork extent record}
   pub filResrv:      u32, // LongInt     {reserved}
}

#[derive(FileReadable)]
#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct CdrThdRec {
   pub thdResrv:      [u32; 2], // ARRAY[1..2] OF LongInt; {reserved}
   pub thdParID:      u32,      // LongInt;    {parent ID for this directory}
   pub thdCName:      PString,  // Str31;     {name of this directory}
}

// CdrFThdRec and CdrThdRec is the same, also according to an explicit comment
// in Inside Macintosh. Reuse CdrThdRec for simplicity, but keep CdrFThdRec for
// reference
//
// #[derive(FileReadable)]
// #[derive(Debug)]
// #[allow(non_snake_case)] // This struct comes from old Mac structs
// pub struct CdrFThdRec {
//    fthdResrv:     [u32; 2], // ARRAY[1..2] OF LongInt; {reserved}
//    fthdParID:     u32,      // LongInt;    {parent ID for this file}
//    fthdCName:     PString,  // Str31;      {name of this file}
// }

#[derive(Debug)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub enum CatDataRec {
    CdrDirRec(CdrDirRec),
    CdrFilRec(CdrFilRec),
    CdrThdRec(CdrThdRec),
    CdrFThdRec(CdrThdRec),
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
            4 => CatDataRec::CdrFThdRec(CdrThdRec::read(rdr)?),
            _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
        })
    }
}

impl CatDataRec {
   pub fn is_object(&self) -> bool {
      match self {
         CatDataRec::CdrDirRec(_) => true,
         CatDataRec::CdrFilRec(_) => true,
         CatDataRec::CdrThdRec(_) => false,
         CatDataRec::CdrFThdRec(_) => false
      }
   }
}