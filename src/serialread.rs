

use byteorder::{BigEndian, ReadBytesExt};
use std::io::{
    Cursor,
    Seek,
    SeekFrom
};

pub trait SerialRead : std::marker::Sized {
    fn read( rdr : &mut SerialReadStorage ) -> std::io::Result<Self>;
}

impl SerialRead for u8 {
    fn read( rdr : &mut SerialReadStorage ) -> std::io::Result<Self> {
        rdr.read_u8()
    }
}

impl SerialRead for i8 {
    fn read( rdr : &mut SerialReadStorage ) -> std::io::Result<Self> {
        rdr.read_i8()
    }
}

impl SerialRead for u16 {
    fn read( rdr : &mut SerialReadStorage ) -> std::io::Result<Self> {
        rdr.read_u16()
    }
}

impl SerialRead for i16 {
    fn read( rdr : &mut SerialReadStorage ) -> std::io::Result<Self> {
        rdr.read_i16()
    }
}

impl SerialRead for u32 {
    fn read( rdr : &mut SerialReadStorage ) -> std::io::Result<Self> {
        rdr.read_u32()
    }
}

impl SerialRead for i32 {
    fn read( rdr : &mut SerialReadStorage ) -> std::io::Result<Self> {
        rdr.read_i32()
    }
}


pub struct SerialReadStorage {
    block : Cursor<Vec<u8>>,
    len_stack : Vec<u64>
}

impl From<Vec<u8>> for SerialReadStorage {
    fn from(vec : Vec<u8>) -> SerialReadStorage {
        SerialReadStorage {
            block: Cursor::new(vec),
            len_stack: vec![]
        }
    }
}

fn pad_to_wordlen(len: u64, wordlen: u64) -> u64 {
    len + ((wordlen - 1) ^ ((len + wordlen - 1) & (wordlen - 1)))
}

impl SerialReadStorage {
    pub fn seek(&mut self, offset : u64) {
        self.block.seek(SeekFrom::Start(offset as u64)).unwrap();
    }

    pub fn size(&self) -> u64 {
        self.block.get_ref().len() as u64
    }

    pub fn length_start(&mut self, len : u64) -> &mut Self {
        let cur_pos = self.block.seek(SeekFrom::Current(0)).unwrap();
        self.len_stack.push(cur_pos + len);
        self
    }

    pub fn length_end(&mut self) -> &mut Self {
        let pos = self.len_stack.pop().unwrap();
        self.seek(pos);
        self
    }

    pub fn align(&mut self, wordlength : u64) -> &mut Self {
        let cur_pos = self.block.seek(SeekFrom::Current(0)).unwrap();
        self.seek(pad_to_wordlen(cur_pos, wordlength));
        self
    }

    pub fn pad(&mut self, bytes : i64) -> &mut Self {
        self.block.seek(SeekFrom::Current(bytes)).unwrap();
        self
    }

    pub fn read_u8(&mut self) -> std::io::Result<u8> {
        self.block.read_u8()
    }
    pub fn read_i8(&mut self) -> std::io::Result<i8> {
        self.block.read_i8()
    }
    pub fn read_u16(&mut self) -> std::io::Result<u16> {
        self.block.read_u16::<BigEndian>()
    }
    pub fn read_i16(&mut self) -> std::io::Result<i16> {
        self.block.read_i16::<BigEndian>()
    }
    pub fn read_u32(&mut self) -> std::io::Result<u32> {
        self.block.read_u32::<BigEndian>()
    }
    pub fn read_i32(&mut self) -> std::io::Result<i32> {
        self.block.read_i32::<BigEndian>()
    }

    pub fn sub_reader(&self, offset : u64, len : u64) -> SerialReadStorage {
        let inner = self.block.get_ref();
        SerialReadStorage::from(Vec::from(&inner[offset as usize..(offset+len) as usize]))
    }

    #[cfg(test)]
    pub fn to_vec(self) -> Vec<u8> {
        self.block.into_inner()
    }
}

impl std::fmt::Debug for SerialReadStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.block.get_ref();
        let len = inner.len() as usize;
        let pos = self.block.position() as usize;
        let start = if pos < 16 { 0 } else { pos-16 };
        let end = if pos+16 > len { len } else { pos+16 };

        write!(f, "Reader @{} len={}: [...", pos, len)?;
        for b in &inner[start..pos] {
            write!(f, " {:02X}", b)?;
        }
        write!(f, " * ")?;
        for b in &inner[pos..end] {
            write!(f, " {:02X}", b)?;
        }
        write!(f, "...]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{SerialReadStorage, SerialRead};

    use super::pad_to_wordlen;
    #[test]
    fn pad_to_wordlen_2() {
        assert_eq!(0, pad_to_wordlen(0, 2));
        assert_eq!(2, pad_to_wordlen(1, 2));
        assert_eq!(2, pad_to_wordlen(2, 2));
        assert_eq!(4, pad_to_wordlen(3, 2));
        assert_eq!(4, pad_to_wordlen(4, 2));
        assert_eq!(6, pad_to_wordlen(5, 2));
        assert_eq!(6, pad_to_wordlen(6, 2));
        assert_eq!(8, pad_to_wordlen(7, 2));
        assert_eq!(8, pad_to_wordlen(8, 2));
    }
    #[test]
    fn pad_to_wordlen_4() {
        assert_eq!(0, pad_to_wordlen(0, 4));
        assert_eq!(4, pad_to_wordlen(1, 4));
        assert_eq!(4, pad_to_wordlen(2, 4));
        assert_eq!(4, pad_to_wordlen(3, 4));
        assert_eq!(4, pad_to_wordlen(4, 4));
        assert_eq!(8, pad_to_wordlen(5, 4));
        assert_eq!(8, pad_to_wordlen(6, 4));
        assert_eq!(8, pad_to_wordlen(7, 4));
        assert_eq!(8, pad_to_wordlen(8, 4));
    }

    #[derive(SerialRead)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    struct TestStruct {
        a : u8,
        b : u16,
        c : [u16; 2],
    }



    #[test]
    fn read_struct() {
        let mut rdr = SerialReadStorage::from(vec![1,2,3,4,5,6,7]);
        let actual : TestStruct = SerialRead::read(&mut rdr).unwrap();
        assert_eq!(
            actual,
            TestStruct {
                a : 0x01,
                b : 0x0203,
                c : [ 0x0405, 0x0607 ]
            }
        );
    }

    #[test]
    fn read_seq() {
        let mut rdr = SerialReadStorage::from(vec![1,2,3,4,5,6,7,2,2,3,4,5,6,7]);
        
        let actual : TestStruct = SerialRead::read(&mut rdr).unwrap();
        assert_eq!(
            actual,
            TestStruct {
                a : 0x01,
                b : 0x0203,
                c : [ 0x0405, 0x0607 ]
            }
        );
        
        let actual : TestStruct = SerialRead::read(&mut rdr).unwrap();
        assert_eq!(
            actual,
            TestStruct {
                a : 0x02,
                b : 0x0203,
                c : [ 0x0405, 0x0607 ]
            }
        );
    }

    #[derive(SerialRead)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    struct TestSuperStruct {
        a : TestStruct,
        b : TestStruct
    }

    #[test]
    fn read_recursive() {
        let mut rdr = SerialReadStorage::from(vec![1,2,3,4,5,6,7,2,2,3,4,5,6,7]);
        
        let actual : TestSuperStruct = SerialRead::read(&mut rdr).unwrap();
        assert_eq!(
            actual,
            TestSuperStruct {
                a: TestStruct {
                    a : 0x01,
                    b : 0x0203,
                    c : [ 0x0405, 0x0607 ]
                },
                b: TestStruct {
                    a : 0x02,
                    b : 0x0203,
                    c : [ 0x0405, 0x0607 ]
                }
            }
        );
    }


    #[derive(SerialRead)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    struct TestSized {
        a : u8,
        #[length_start(3)]
        b : u8,
        #[length_end()]
        c : u8
    }

    #[test]
    fn read_sized() {
        let mut rdr = SerialReadStorage::from(vec![1,2,3,4,5]);
        let actual : TestSized = SerialRead::read(&mut rdr).unwrap();
        assert_eq!(
            actual,
            TestSized {
                a : 1,
                b : 2,
                c : 5
            }
        );
    }
}