use super::super::block::{
    FileReader
};

pub trait FileReadable {
    fn read( rdr : &mut FileReader ) -> Self;
}

impl FileReadable for u8 {
    fn read( rdr : &mut FileReader ) -> Self {
        rdr.read_u8()
    }
}

impl FileReadable for i8 {
    fn read( rdr : &mut FileReader ) -> Self {
        rdr.read_i8()
    }
}

impl FileReadable for u16 {
    fn read( rdr : &mut FileReader ) -> Self {
        rdr.read_u16()
    }
}

impl FileReadable for i16 {
    fn read( rdr : &mut FileReader ) -> Self {
        rdr.read_i16()
    }
}

impl FileReadable for u32 {
    fn read( rdr : &mut FileReader ) -> Self {
        rdr.read_u32()
    }
}

impl FileReadable for i32 {
    fn read( rdr : &mut FileReader ) -> Self {
        rdr.read_i32()
    }
}