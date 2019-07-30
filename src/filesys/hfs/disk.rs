use std::io;

pub struct DiskAdaptor<T: io::Read + io::Seek> (T);

pub trait DiskAccess : std::fmt::Debug {
    fn seek(&mut self, pos : u64) -> io::Result<u64>;
    fn size(&mut self) -> io::Result<u64>;
    fn pos(&mut self) -> io::Result<u64>;
    fn read(&mut self, len : u64) -> io::Result<Vec<u8>>;
}

impl<T> DiskAdaptor<T>
where
T: io::Read + io::Seek {
    // Put in box, so it is always sized, for easier handling. The reason for
    // file adaptor is to have a uniform wrapper for different file types
    pub fn new(f: T) -> Box<DiskAdaptor<T>> {
        Box::new(DiskAdaptor(f))
    }
}

impl<T : std::io::Read + std::io::Seek> std::fmt::Debug for DiskAdaptor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DiskAccess()")
    }
}

impl<T> DiskAccess for DiskAdaptor<T>
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