use std::io;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct FileAdaptor<'storage, T: io::Read + io::Seek + std::fmt::Debug> (&'storage mut T);

pub trait FileAccess : std::fmt::Debug {
    fn seek(&mut self, pos : u64) -> io::Result<u64>;
    fn size(&mut self) -> io::Result<u64>;
    fn pos(&mut self) -> io::Result<u64>;
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_i16(&mut self) -> io::Result<i16>;
    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_i32(&mut self) -> io::Result<i32>;
    fn read_u32(&mut self) -> io::Result<u32>;
    fn read_vec(&mut self, len : usize) -> io::Result<Vec<u8>>;
}

impl<'storage, T> FileAdaptor<'storage, T>
where
T: io::Read + io::Seek + std::fmt::Debug {
    pub fn new(f: &'storage mut T) -> FileAdaptor<'storage, T> {
        FileAdaptor(f)
    }
}

impl<'storage, T> FileAccess for FileAdaptor<'storage, T>
where
T: io::Read + io::Seek + std::fmt::Debug {
    fn seek(&mut self, pos : u64) -> io::Result<u64> {
        self.0.seek(io::SeekFrom::Start(pos))
    }
    fn size(&mut self) -> io::Result<u64> {
        self.0.seek(io::SeekFrom::End(0))
    }
    fn pos(&mut self) -> io::Result<u64> {
        self.0.seek(io::SeekFrom::Current(0))
    }
    fn read_u8(&mut self) -> io::Result<u8> {
        self.0.read_u8()
    }
    fn read_i16(&mut self) -> io::Result<i16> {
        self.0.read_i16::<BigEndian>()
    }
    fn read_u16(&mut self) -> io::Result<u16> {
        self.0.read_u16::<BigEndian>()
    }
    fn read_i32(&mut self) -> io::Result<i32> {
        self.0.read_i32::<BigEndian>()
    }
    fn read_u32(&mut self) -> io::Result<u32> {
        self.0.read_u32::<BigEndian>()
    }
    fn read_vec(&mut self, len : usize) -> io::Result<Vec<u8>> {
        let mut bufv : Vec<u8> = Vec::with_capacity(len);
        for _ in 0..len {
            bufv.push(self.read_u8()?);
        }
        Ok(bufv)
    }
}