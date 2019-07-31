use super::{
    BlockAccess,
    types::common::ExtDataRec
};

use std::io::{
    Seek,
    SeekFrom,
    Read,
    Result,
    Error,
    ErrorKind
};

#[derive(Debug)]
pub struct FileIO {
    storage: BlockAccess,
    size: u64,
    rec: ExtDataRec,
    cur: u64
}

impl FileIO {
    pub fn open(storage: BlockAccess, size: u64, rec: ExtDataRec) -> FileIO {
        FileIO {
            storage,
            size,
            rec,
            cur: 0
        }
    }
}

impl Seek for FileIO {
    fn seek(&mut self, from: SeekFrom) -> Result<u64> {
        let newpos = match from {
            SeekFrom::Start(offset) => offset as i64,
            SeekFrom::Current(offset) => offset as i64 + self.cur as i64,
            SeekFrom::End(offset) => offset as i64 + self.size as i64,
        };

        if newpos < 0 || newpos >= self.size as i64 {
            Err(Error::from(ErrorKind::InvalidInput))
        } else {
            self.cur = newpos as u64;
            Ok(self.cur as u64)
        }
    }
}

impl Read for FileIO {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let data_left: i64 = self.size as i64 - self.cur as i64;
        let buf_len: i64 = buf.len() as i64;
        let to_read = if data_left > buf_len { buf_len } else { data_left };

        let mut reader = self.storage.read_extdatarec(
            &self.rec,
            self.cur,
            to_read as u64
        )?;

        for i in 0..to_read as usize {
            if let Ok(byte) = reader.read_u8() {
                self.cur += 1;
                buf[i] = byte;
            }
        }
        Ok(to_read as usize)
    }
}