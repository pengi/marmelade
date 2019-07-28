use super::super::block::FileReader;
use super::FileReadable;

#[derive(Debug)]
#[derive(FileReadable)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct NodeDescriptor {
    ndFLink:       u32, //LongInt;       {forward link}
    ndBLink:       u32, //LongInt;       {backward link}
    ndType:        i8,  //SignedByte;    {node type}
    ndNHeight:     i8,  //SignedByte;    {node level}
    ndNRecs:       u16, //Integer;       {number of records in node}
    ndResv2:       u16, //Integer;       {reserved}
}


#[derive(Debug)]
#[derive(FileReadable)]
#[allow(non_snake_case)] // This struct comes from old Mac structs
pub struct BTHdrRec {
   bthDepth:      u16, //Integer;    {current depth of tree}
   bthRoot:       u32, //LongInt;    {number of root node}
   bthNRecs:      u32, //LongInt;    {number of leaf records in tree}
   bthFNode:      u32, //LongInt;    {number of first leaf node}
   bthLNode:      u32, //LongInt;    {number of last leaf node}
   bthNodeSize:   u16, //Integer;    {size of a node}
   bthKeyLen:     u16, //Integer;    {maximum length of a key}
   bthNNodes:     u32, //LongInt;    {total number of nodes in tree}
   bthFree:       u32, //LongInt;    {number of free nodes}
   bthResv:       [u32; 19] //ARRAY[1..76] OF SignedByte;   {reserved}
}