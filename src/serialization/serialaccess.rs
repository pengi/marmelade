use std::io;
use std::cell::RefCell;

use super::SerialReadStorage;

pub struct SerialAdaptor<T: io::Read + io::Seek> (RefCell<T>);

pub trait SerialAccess : std::fmt::Debug {
    fn size(&self) -> io::Result<u64>;
    fn read(&self, pos : u64, len : u64) -> io::Result<SerialReadStorage>;
}

impl<T> SerialAdaptor<T>
where
T: io::Read + io::Seek {
    // Put in box, so it is always sized, for easier handling. The reason for
    // file adaptor is to have a uniform wrapper for different file types
    pub fn new(f: T) -> Box<SerialAdaptor<T>> {
        Box::new(SerialAdaptor(RefCell::new(f)))
    }
}

impl<T : std::io::Read + std::io::Seek> std::fmt::Debug for SerialAdaptor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SerialAccess()")
    }
}

impl<T> SerialAccess for SerialAdaptor<T>
where
T: io::Read + io::Seek {
    fn size(&self) -> io::Result<u64> {
        let mut storage = self.0.borrow_mut();
        storage.seek(io::SeekFrom::End(0))
    }
    fn read(&self, pos : u64, len : u64) -> io::Result<SerialReadStorage> {
        let mut bufv : Vec<u8> = Vec::with_capacity(len as usize);
        let mut storage = self.0.borrow_mut();
        storage.seek(io::SeekFrom::Start(pos))?;
        for _ in 0..len {
            let mut arr = [0u8;1];
            storage.read_exact(&mut arr)?;
            bufv.push(arr[0]);
        }
        Ok(SerialReadStorage::from(bufv))
    }
}