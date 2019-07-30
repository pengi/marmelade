#[macro_use]
extern crate clap;

use marmelade::filesys::hfs;
use marmelade::filesys::hfs::DiskAdaptor;
use marmelade::filesys::hfs::{
    HfsObjRef,
    HfsDirIter
};
use std::fs;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Max Sikström <max@pengi.se>")
        (about: "Makes toasters fly - run old stuff on new machines")
        (@arg img: +required -i --image +takes_value "Image file")
        (@arg file: -f --file +takes_value "File to read")
    ).get_matches();

    let imgfile = matches.value_of("img").unwrap();
    let img = fs::File::open(imgfile).unwrap();
    let fa = DiskAdaptor::new(img);
    let fs = hfs::HfsImage::from(fa).unwrap();

    if let Some(file) = matches.value_of("file") {
        let file = fs.locate(file);
        println!("File: {:#?}", file);
    } else {
        print_files(fs.open_root(), 0);
    }
}


fn print_files(dir: HfsDirIter, indent: usize) {
    let indstr = String::from("    ").repeat(indent);

    for obj in dir {
        match obj {
            HfsObjRef::FileRef(file) => {
                let (data_size, rsrc_size) = file.get_size();
                println!("{}{:?} (size: {}/{})", indstr, file.get_name(), data_size, rsrc_size);
            },
            HfsObjRef::DirRef(dir) => {
                println!("{}{:?}:", indstr, dir.get_name());
                print_files(dir.open(), indent+1);
            }
        }
    }
}