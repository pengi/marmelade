use super::{
    blockaccess::BlockAccess,
    types::{btree::BTHdrRec, btree::NodeDescriptor, common::ExtDataRec, FileReadable, FileReader},
};

use std::marker::PhantomData;

#[derive(Debug)]
struct BTreeNode {
    nd: NodeDescriptor,
    recs: Vec<FileReader>,
}

impl BTreeNode {
    pub fn new(rdr: &mut FileReader) -> std::io::Result<BTreeNode> {
        rdr.seek(0);

        let nd = NodeDescriptor::read(rdr)?;
        let mut recs: Vec<FileReader> = Vec::with_capacity(nd.ndNRecs as usize);

        let size = rdr.size();

        for i in 0..nd.ndNRecs {
            rdr.seek(size - 4 - 2 * (i as u64));
            let idx_end = rdr.read_u16()?;
            let idx_start = rdr.read_u16()?;
            recs.push(rdr.sub_reader(idx_start as u64, (idx_end - idx_start) as u64));
        }

        Ok(BTreeNode { nd, recs })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FileReader,
        FileReadable,
        BTreeNode
    };

    #[test]
    fn unpack_tree_node() {
        let mut rdr = FileReader::from(vec![
            0,0,1,0, // nd.ndFLink
            0,0,2,0, // nd.ndBLink
            12, // nd.ndType
            7, // nd.ndNHeight
            0, 3, // nd.ndNRecs
            0, 0, // nd.ndResv2
            0,1,2,3,4,5,6, 7, // Data
            100,101,102,103,104,105,106,107, // Unused
            0, 22, 0, 18, 0, 16, 0, 14 // Index
        ]);
        let mut bt = BTreeNode::new(&mut rdr).unwrap();
        assert_eq!(bt.nd.ndFLink, 256);
        assert_eq!(bt.nd.ndBLink, 512);
        assert_eq!(bt.nd.ndType, 12);
        assert_eq!(bt.nd.ndNHeight, 7);
        assert_eq!(bt.nd.ndNRecs, 3);
        assert_eq!(bt.nd.ndResv2, 0);

        assert_eq!(bt.recs.len(), 3);

        assert_eq!(bt.recs[0].size(), 2);
        assert_eq!(u16::read(&mut bt.recs[0]).unwrap(), 0x0001u16);

        assert_eq!(bt.recs[1].size(), 2);
        assert_eq!(u16::read(&mut bt.recs[1]).unwrap(), 0x0203u16);

        assert_eq!(bt.recs[2].size(), 4);
        assert_eq!(u32::read(&mut bt.recs[2]).unwrap(), 0x04050607u32);
        
    }
}


#[derive(Debug)]
pub struct BTreeHeaderNode {
    nd: NodeDescriptor,
    header: BTHdrRec,
}

impl BTreeHeaderNode {
    pub fn new(rdr: &mut FileReader) -> std::io::Result<BTreeHeaderNode> {
        Ok(BTreeHeaderNode::from(BTreeNode::new(rdr)?))
    }
}

impl From<BTreeNode> for BTreeHeaderNode {
    fn from(node: BTreeNode) -> BTreeHeaderNode {
        let mut node = node;
        assert_eq!(node.nd.ndType, 1);
        assert_eq!(node.nd.ndNRecs, 3);

        let header = BTHdrRec::read(&mut node.recs[0]).unwrap();
        // TODO: Read rest of records

        BTreeHeaderNode {
            nd: node.nd,
            header,
        }
    }
}

#[derive(Debug)]
pub struct BTreeLeafNode<K, V>
where
    K: FileReadable + PartialOrd,
    V: FileReadable
{
    nd: NodeDescriptor,
    recs: Vec<(K, V)>,
}

impl<K, V> BTreeLeafNode<K, V>
where
    K: FileReadable + PartialOrd + std::fmt::Debug,
    V: FileReadable + std::fmt::Debug
{
    pub fn new(rdr: &mut FileReader) -> std::io::Result<BTreeLeafNode<K, V>> {
        Ok(BTreeLeafNode::from(BTreeNode::new(rdr)?))
    }
}

impl<K, V> From<BTreeNode> for BTreeLeafNode<K, V>
where
    K: FileReadable + PartialOrd + std::fmt::Debug,
    V: FileReadable + std::fmt::Debug
{
    fn from(node: BTreeNode) -> BTreeLeafNode<K, V> {
        assert_eq!(node.nd.ndType, -1i8);

        let mut recs = Vec::with_capacity(node.recs.len());

        println!("{:#?}", node.nd);

        for mut rdr in node.recs {
            if let Ok(key) = K::read(&mut rdr) {
                rdr.align(2);
                if let Ok(val) = V::read(&mut rdr) {
                    println!("{:#?} {:#?}", key, val);
                    recs.push((key, val));
                }
            }
            println!("{:#?}", rdr);
        }

        BTreeLeafNode { nd: node.nd, recs }
    }
}

pub struct BTreeIter<K, V>
where
    K: FileReadable + PartialOrd,
    V: FileReadable
{
    key_type: PhantomData<K>,
    value_type: PhantomData<V>,
}

impl<K, V> std::iter::Iterator for BTreeIter<K, V>
where
    K: FileReadable + PartialOrd,
    V: FileReadable
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Debug)]
pub struct BTree<'storage, K, V>
where
    K: FileReadable + PartialOrd,
    V: FileReadable
{
    storage: BlockAccess<'storage>,
    datarec: ExtDataRec,
    header: BTreeHeaderNode,

    key_type: PhantomData<K>,
    value_type: PhantomData<V>,
}

impl<'storage, K, V> BTree<'storage, K, V>
where
    K: FileReadable + PartialOrd + std::fmt::Debug,
    V: FileReadable + std::fmt::Debug
{
    pub fn new(
        storage: &BlockAccess<'storage>,
        datarec: &ExtDataRec,
    ) -> std::io::Result<BTree<'storage, K, V>> {
        let storage = storage.clone();
        let datarec = datarec.clone();

        let mut headerblock = storage.read_extdatarec(&datarec, 0, 512)?;
        let header = BTreeHeaderNode::new(&mut headerblock)?;

        Ok(BTree {
            storage,
            datarec,
            header,
            key_type: PhantomData,
            value_type: PhantomData,
        })
    }

    pub fn iter(&self) -> BTreeIter<K, V> {
        let mut lnblk = self.storage.read_extdatarec(
            &self.datarec,
            self.header.header.bthFNode as u64 * 512,
            512,
        ).unwrap();
        let ln = BTreeLeafNode::<K, V>::new(&mut lnblk);

        println!("{:#?}", ln);

        BTreeIter {
            key_type: PhantomData,
            value_type: PhantomData,
        }
    }
}
