use crate::serialization::{SerialReadStorage, SerialRead};

#[derive(Debug)]
#[derive(Clone)]
#[derive(SerialRead)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct ExtDescriptor {
    pub xdrStABN: u16,    // first allocation block
    pub xdrNumABlks: i16, // number of allocation blocks
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(SerialRead)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct ExtDataRec(
    pub [ExtDescriptor; 3]
);
