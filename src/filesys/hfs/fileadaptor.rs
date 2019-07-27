use std::io;
use byteorder::{BigEndian, ReadBytesExt};

pub struct FileBlock {
    data : Vec<u8>
}

impl FileBlock {
    pub fn read_u8(&self, offset: usize) -> u8 {
        self.data[offset]
    }
    pub fn read_i8(&self, offset: usize) -> i8 {
        self.data[offset] as i8
    }
    pub fn read_i16(&self, offset: usize) -> i16 {
        let mut f = io::Cursor::new(self.data.get(offset..offset+2).unwrap());
        f.read_i16::<BigEndian>().unwrap()
    }
    pub fn read_u16(&self, offset: usize) -> u16 {
        let mut f = io::Cursor::new(self.data.get(offset..offset+2).unwrap());
        f.read_u16::<BigEndian>().unwrap()
    }
    pub fn read_i32(&self, offset: usize) -> i32 {
        let mut f = io::Cursor::new(self.data.get(offset..offset+4).unwrap());
        f.read_i32::<BigEndian>().unwrap()
    }
    pub fn read_u32(&self, offset: usize) -> u32 {
        let mut f = io::Cursor::new(self.data.get(offset..offset+4).unwrap());
        f.read_u32::<BigEndian>().unwrap()
    }
    pub fn read_pstr(&self, offset: usize, len: usize) -> String {
        let actual_len = *self.data.get(offset).unwrap() as usize;
        let fetch_len = if len < actual_len { len } else { actual_len };
        String::from(String::from_utf8_lossy(self.data.get(offset+1..offset+fetch_len+1).unwrap()))
    }
    pub fn read_vec(&self, offset: usize, len: usize) -> Vec<u8> {
        Vec::from(&self.data[offset..offset+len])
    }
}

impl std::fmt::Debug for FileBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<FileBlock>")?;
        Ok(())
    }
}

pub struct FileAdaptor<'storage, T: io::Read + io::Seek> (&'storage mut T);

pub trait FileAccess : std::fmt::Debug {
    fn seek(&mut self, pos : u64) -> io::Result<u64>;
    fn size(&mut self) -> io::Result<u64>;
    fn pos(&mut self) -> io::Result<u64>;
    fn read_vec(&mut self, len : usize) -> io::Result<FileBlock>;
}

impl<'storage, T> FileAdaptor<'storage, T>
where
T: io::Read + io::Seek {
    pub fn new(f: &'storage mut T) -> FileAdaptor<'storage, T> {
        FileAdaptor(f)
    }
}

impl<'storage, T : std::io::Read + std::io::Seek> std::fmt::Debug for FileAdaptor<'storage, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileAccess()")
    }
}

impl<'storage, T> FileAccess for FileAdaptor<'storage, T>
where
T: io::Read + io::Seek {
    fn seek(&mut self, pos : u64) -> io::Result<u64> {
        self.0.seek(io::SeekFrom::Start(pos))
    }
    fn size(&mut self) -> io::Result<u64> {
        self.0.seek(io::SeekFrom::End(0))
    }
    fn pos(&mut self) -> io::Result<u64> {
        self.0.seek(io::SeekFrom::Current(0))
    }
    fn read_vec(&mut self, len : usize) -> io::Result<FileBlock> {
        let mut bufv : Vec<u8> = Vec::with_capacity(len);
        for _ in 0..len {
            let mut arr = [0u8;1];
            self.0.read_exact(&mut arr)?;
            bufv.push(arr[0]);
        }
        Ok(FileBlock{data: bufv})
    }
}

pub struct FileBlockSeqReader<'storage> {
    block : &'storage FileBlock,
    offset : usize
}

impl<'storage> FileBlockSeqReader<'storage> {
    pub fn from(block : &'storage FileBlock, offset : usize) -> FileBlockSeqReader {
        FileBlockSeqReader { block, offset }
    }

    pub fn read_u8(&mut self) -> u8 {
        let res = self.block.read_u8(self.offset);
        self.offset += 1;
        res
    }
    pub fn read_i8(&mut self) -> i8 {
        let res = self.block.read_i8(self.offset);
        self.offset += 1;
        res
    }
    pub fn read_i16(&mut self) -> i16 {
        let res = self.block.read_i16(self.offset);
        self.offset += 2;
        res
    }
    pub fn read_u16(&mut self) -> u16 {
        let res = self.block.read_u16(self.offset);
        self.offset += 2;
        res
    }
    pub fn read_i32(&mut self) -> i32 {
        let res = self.block.read_i32(self.offset);
        self.offset += 4;
        res
    }
    pub fn read_u32(&mut self) -> u32 {
        let res = self.block.read_u32(self.offset);
        self.offset += 4;
        res
    }
    pub fn read_pstr(&mut self, len: usize) -> String {
        let res = self.block.read_pstr(self.offset, len);
        self.offset += 1+len;
        res
    }
}