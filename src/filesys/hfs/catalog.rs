use super::{
    types::{
        common::ExtDataRec,
        catalog::{
            CatKeyRec,
            CatDataRec
        }
    },
    blockaccess::BlockAccess,
    btree::BTree
};

#[derive(Debug)]
pub struct Catalog<'storage> {
    btree: BTree<'storage, CatKeyRec, CatDataRec>
}

impl<'storage> Catalog<'storage> {
    pub fn new(storage : &BlockAccess<'storage>, datarec: &ExtDataRec) -> std::io::Result<Catalog<'storage>> {
        let btree = BTree::new(storage, datarec)?;
        Ok(Catalog{
            btree
        })
    }

    pub fn list_files(&self) {
        for (key, val) in self.btree.iter().unwrap() {
            println!("{:?} {:?}", key, val);
        }
    }
}