use super::types::{
    FileReader,
    FileReadable,
    btree::NodeDescriptor,
    btree::BTHdrRec
};

#[derive(Debug)]
struct BTreeNode {
    nd : NodeDescriptor,
    recs : Vec<FileReader>
}

impl BTreeNode {
    pub fn new(rdr: &mut FileReader) -> BTreeNode {
        rdr.seek(0);

        let nd = NodeDescriptor::read(rdr);
        let mut recs : Vec<FileReader> = Vec::with_capacity(nd.ndNRecs as usize);

        let size = rdr.size();

        for i in 0..nd.ndNRecs {
            rdr.seek(size-4-4*(i as u64));
            let idx_end = rdr.read_u16();
            let idx_start = rdr.read_u16();
            recs.push(rdr.sub_reader(idx_start as u64, (idx_end-idx_start) as u64));
        }


        BTreeNode{ nd, recs }
    }
}

#[derive(Debug)]
pub struct BTreeHeaderNode {
    nd : NodeDescriptor,
    header : BTHdrRec
}

impl BTreeHeaderNode {
    pub fn new(rdr: &mut FileReader) -> BTreeHeaderNode {
        BTreeHeaderNode::from(BTreeNode::new(rdr))
    }
}

impl From<BTreeNode> for BTreeHeaderNode {
    fn from(node : BTreeNode) -> BTreeHeaderNode {
        let mut node = node;
        assert_eq!(node.nd.ndType, 1);
        assert_eq!(node.nd.ndNRecs, 3);

        let header = BTHdrRec::read(&mut node.recs[0]);
        // TODO: Read rest of records

        BTreeHeaderNode {
            nd: node.nd,
            header
        }
    }
}