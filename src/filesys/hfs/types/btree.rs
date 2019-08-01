use crate::serialization::{SerialReadStorage, SerialRead};

#[derive(Debug)]
#[derive(SerialRead)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct NodeDescriptor {
    pub ndFLink:       u32, //LongInt;       {forward link}
    pub ndBLink:       u32, //LongInt;       {backward link}
    pub ndType:        i8,  //SignedByte;    {node type}
    pub ndNHeight:     i8,  //SignedByte;    {node level}
    pub ndNRecs:       u16, //Integer;       {number of records in node}
    pub ndResv2:       u16, //Integer;       {reserved}
}


#[derive(Debug)]
#[derive(SerialRead)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct BTHdrRec {
   pub bthDepth:      u16, //Integer;    {current depth of tree}
   pub bthRoot:       u32, //LongInt;    {number of root node}
   pub bthNRecs:      u32, //LongInt;    {number of leaf records in tree}
   pub bthFNode:      u32, //LongInt;    {number of first leaf node}
   pub bthLNode:      u32, //LongInt;    {number of last leaf node}
   pub bthNodeSize:   u16, //Integer;    {size of a node}
   pub bthKeyLen:     u16, //Integer;    {maximum length of a key}
   pub bthNNodes:     u32, //LongInt;    {total number of nodes in tree}
   pub bthFree:       u32, //LongInt;    {number of free nodes}
   pub bthResv:       [u32; 19] //ARRAY[1..76] OF SignedByte;   {reserved}
}
