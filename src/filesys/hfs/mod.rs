mod types;
mod blockaccess;
mod btree;
mod catalog;
mod disk;
mod fileio;

use std::io;

use disk::DiskAccess;
pub use disk::DiskAdaptor;

use crate::serialread::{SerialReadStorage, SerialRead};

use types::{
    mdb::MDB,
    catalog::{
        CatDataRec,
        CatKeyRec,
        CdrFilRec,
        CdrDirRec
    }
};
use blockaccess::BlockAccess;

use catalog::{
    Catalog,
    CatalogIterator
};

pub use fileio::FileIO;

#[derive(Debug)]
pub struct HfsImage
{
    storage: BlockAccess,
    mdb: MDB,
    pub catalog: Catalog
}

impl HfsImage
{
    pub fn from(storage: Box<dyn DiskAccess>) -> io::Result<HfsImage> {
        let mut storage = storage;
        // let size = storage.size()?;

        // Bootstrap with getting header, to get block size information
        storage.seek(2*512)?;
        let mut mdb_block : SerialReadStorage = SerialReadStorage::from(storage.read(512)?);
        let mdb = MDB::read(&mut mdb_block)?;

        // Set up block access
        let storage = BlockAccess::new(storage, mdb.drAlBlSt as u64, mdb.drAlBlkSiz as u64);

        let catalog = Catalog::new(&storage, &mdb.drCTExtRec)?;

        Ok(HfsImage {storage, mdb, catalog})
    }

    pub fn open_root<'img>(&'img self) -> HfsDirIter<'img> {
        HfsDirIter {
            img: self,
            iter: self.catalog.dir(2)
        }
    }

    fn open_dir<'img>(&'img self, dir: u32) -> HfsDirIter<'img> {
        HfsDirIter {
            img: self,
            iter: self.catalog.dir(dir)
        }
    }

    pub fn locate<'img>(&'img self, path: &str) -> Option<HfsObjRef<'img>> {
        let mut path: Vec<&str> = path.split(':').collect();

        let plast = path.pop()?;

        let mut iter = self.open_root();
        for part in path {
            let obj = iter.find(|objr| objr.get_name() == part)?;
            if let HfsObjRef::DirRef(dir) = obj {
                iter = dir.open();
            } else {
                return None;
            }
        }
        iter.find(|objr| objr.get_name() == plast)
    }
}


pub struct HfsDirIter<'img> {
    img: &'img HfsImage,
    iter: CatalogIterator<'img>
}

#[derive(Debug)]
pub struct HfsFileRef<'img> {
    img: &'img HfsImage,
    key: CatKeyRec,
    fr: CdrFilRec
}

#[derive(Debug)]
pub struct HfsDirRef<'img> {
    img: &'img HfsImage,
    key: CatKeyRec,
    dr: CdrDirRec
}

#[derive(Debug)]
pub enum HfsObjRef<'img> {
    FileRef(HfsFileRef<'img>),
    DirRef(HfsDirRef<'img>)
}

impl<'img> std::iter::Iterator for HfsDirIter<'img> {
    type Item = HfsObjRef<'img>;

    fn next(&mut self) -> Option<HfsObjRef<'img>> {
        let (key, elem) = self.iter.next()?;

        match elem {
            CatDataRec::CdrFilRec(fr) => {
                Some(HfsObjRef::FileRef(HfsFileRef{ img: self.img, key, fr }))
            },
            CatDataRec::CdrDirRec(dr) => {
                Some(HfsObjRef::DirRef(HfsDirRef{ img: self.img, key, dr }))
            },
            _ => None
        }
    }
}

impl<'img> HfsFileRef<'img> {
    pub fn get_name(&self) -> String {
        String::from(&self.key.ckrCName)
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.fr.filLgLen, self.fr.filRLgLen)
    }

    pub fn open(&self) -> FileIO {
        FileIO::open(
            self.img.storage.clone(),
            self.fr.filLgLen as u64,
            self.fr.filExtRec.clone()
        )
    }

    pub fn open_rsrc(&self) -> FileIO {
        FileIO::open(
            self.img.storage.clone(),
            self.fr.filRLgLen as u64,
            self.fr.filRExtRec.clone()
        )
    }
}

impl<'img> HfsDirRef<'img> {
    pub fn get_name(&self) -> String {
        String::from(&self.key.ckrCName)
    }

    pub fn open(&self) -> HfsDirIter<'img> {
        self.img.open_dir(self.dr.dirDirID)
    }
}

impl<'img> HfsObjRef<'img> {
    pub fn get_name(&self) -> String {
        match self {
            HfsObjRef::FileRef(fr) => fr.get_name(),
            HfsObjRef::DirRef(dr) => dr.get_name()
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            HfsObjRef::FileRef(_) => false,
            HfsObjRef::DirRef(_) => true
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            HfsObjRef::FileRef(_) => true,
            HfsObjRef::DirRef(_) => false
        }
    }
}