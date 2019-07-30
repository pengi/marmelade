use super::{
    types::{
        common::{
            ExtDataRec
        },
        catalog::{
            CatKeyRec,
            CatDataRec
        }
    },
    blockaccess::BlockAccess,
    btree::{
        BTree,
        BTreeIter
    }
};

#[derive(Debug)]
pub struct Catalog {
    btree: BTree<CatKeyRec, CatDataRec>
}

impl Catalog {
    pub fn new(storage : &BlockAccess, datarec: &ExtDataRec) -> std::io::Result<Catalog> {
        let btree = BTree::new(storage, datarec)?;
        Ok(Catalog{
            btree
        })
    }

    pub fn dir<'iter>(&'iter self, dir: u32) -> CatalogIterator<'iter> {
        CatalogIterator {
            iter: self.btree.iter(),
            dir
        }
    }
}

pub struct CatalogIterator<'iter> {
    iter: BTreeIter<'iter, CatKeyRec, CatDataRec>,
    dir: u32
}

impl<'iter> std::iter::Iterator for CatalogIterator<'iter> {
    type Item = (CatKeyRec, CatDataRec);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((key, data)) = self.iter.next() {
                if data.is_object() && key.ckrParID == self.dir {
                    break Some((key, data))
                }
            } else {
                break None
            }
        }
    }
}
