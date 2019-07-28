use std::io;

pub struct FileAdaptor<'storage, T: io::Read + io::Seek> (&'storage mut T);

pub trait FileAccess : std::fmt::Debug {
    fn seek(&mut self, pos : u64) -> io::Result<u64>;
    fn size(&mut self) -> io::Result<u64>;
    fn pos(&mut self) -> io::Result<u64>;
    fn read(&mut self, len : u64) -> io::Result<Vec<u8>>;
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
    fn read(&mut self, len : u64) -> io::Result<Vec<u8>> {
        let mut bufv : Vec<u8> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let mut arr = [0u8;1];
            self.0.read_exact(&mut arr)?;
            bufv.push(arr[0]);
        }
        Ok(bufv)
    }
}