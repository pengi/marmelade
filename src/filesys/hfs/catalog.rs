use super::{
    types::{
        common::{
            PString,
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

    pub fn dir<'iter>(&'iter self, dir: u32) -> CatalogIterator<'iter, 'storage> {
        CatalogIterator {
            iter: self.btree.iter(),
            dir
        }
    }

    pub fn locate(&self, path: &str) -> Option<CatDataRec> {
        let mut iter = self.dir(2);
        let mut path: Vec<&str> = path.split(':').collect();
        let plast = PString::from(path.pop()?);

        for part in path {
            let ppart = PString::from(part);
            let dir = iter.find(|(key, data)| match data {
                CatDataRec::CdrDirRec(_) => key.ckrCName == ppart,
                CatDataRec::CdrFilRec(_) => false,
                _ => false
            });

            if let Some((_, CatDataRec::CdrDirRec(d))) = dir {
                iter = self.dir(d.dirDirID);
            } else {
                return None;
            }
        }

        if let Some((_, obj)) = iter.find(
            |(key, data)| data.is_object() && key.ckrCName == plast
        ) {
            Some(obj)
        } else {
            None
        }
    }
}

pub struct CatalogIterator<'iter, 'storage> {
    iter: BTreeIter<'iter, 'storage, CatKeyRec, CatDataRec>,
    dir: u32
}

impl<'iter, 'storage> std::iter::Iterator for CatalogIterator<'iter, 'storage> {
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
