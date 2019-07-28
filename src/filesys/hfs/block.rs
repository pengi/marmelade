use byteorder::{BigEndian, ReadBytesExt};

use std::io::{
    Cursor,
    Seek,
    SeekFrom
};

pub struct FileBlock {
    data : Vec<u8>
}

impl std::fmt::Debug for FileBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<FileBlock>")
    }
}

impl FileBlock {
    pub fn from(vec : Vec<u8>) -> FileBlock {
        FileBlock {data:vec}
    }

    pub fn to_reader(self) -> FileReader {
        FileReader {
            block: Cursor::new(self.data),
            next_len: None
        }
    }
}

pub struct FileReader {
    block : Cursor<Vec<u8>>,
    next_len : Option<usize>
}

impl FileReader {
    pub fn seek(&mut self, offset : usize) {
        self.block.seek(SeekFrom::Start(offset as u64)).unwrap();
    }

    pub fn length(&mut self, len : usize) {
        self.next_len = Some(len);
    }

    pub fn read_u8(&mut self) -> u8 { self.block.read_u8().unwrap() }
    pub fn read_i8(&mut self) -> i8 { self.block.read_i8().unwrap() }
    pub fn read_u16(&mut self) -> u16 { self.block.read_u16::<BigEndian>().unwrap() }
    pub fn read_i16(&mut self) -> i16 { self.block.read_i16::<BigEndian>().unwrap() }
    pub fn read_u32(&mut self) -> u32 { self.block.read_u32::<BigEndian>().unwrap() }
    pub fn read_i32(&mut self) -> i32 { self.block.read_i32::<BigEndian>().unwrap() }
}

impl std::fmt::Debug for FileReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<FileReader>")
    }
}
