mod types;
mod blockaccess;
mod btree;
mod catalog;
mod disk;

use std::io;

use disk::DiskAccess;
pub use disk::DiskAdaptor;

use types::{
    FileReader,
    FileReadable,
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
        let mut mdb_block : FileReader = FileReader::from(storage.read(512)?);
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

            iter = obj.open_dir()?;
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

impl<'img> HfsObjRef<'img> {
    pub fn get_name(&self) -> String {
        match self {
            HfsObjRef::FileRef(fr) => String::from(&fr.key.ckrCName),
            HfsObjRef::DirRef(dr) => String::from(&dr.key.ckrCName)
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

    pub fn open_dir(&self) -> Option<HfsDirIter<'img>> {
        if let HfsObjRef::DirRef(dir) = self {
            Some(dir.img.open_dir(dir.dr.dirDirID))
        } else {
            None
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        match self {
            HfsObjRef::FileRef(fr) => (fr.fr.filLgLen, fr.fr.filRLgLen),
            HfsObjRef::DirRef(_dr) => (0, 0)
        }
    }
}