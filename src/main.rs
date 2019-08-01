#[macro_use]
extern crate clap;

use marmelade::serialization::SerialAdaptor;
use marmelade::filesys::hfs::{
    self,
    HfsObjRef,
    HfsDirIter
};
use std::io::Read;
use std::fs;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Max Sikström <max@pengi.se>")
        (about: "Makes toasters fly - run old stuff on new machines")
        (@arg img: +required -i --image +takes_value "Image file")
        (@arg file: -f --file +takes_value "File to read")
        (@arg rsrc: -r --rsrc "Open resource fork instead of data")
    ).get_matches();

    let imgfile = matches.value_of("img").unwrap();
    let img = fs::File::open(imgfile).unwrap();
    let fa = SerialAdaptor::new(img);
    let fs = hfs::HfsImage::from(fa).unwrap();

    if let Some(file) = matches.value_of("file") {
        let use_rsrc = matches.occurrences_of("rsrc") > 0;
        if let Err(err) = open_file(&fs, file, use_rsrc) {
            eprintln!("Error: {}", err);
        }
    } else {
        print_files(fs.open_root(), 0);
    }
}

fn open_file(fs: &hfs::HfsImage, filename: &str, use_rsrc: bool) -> std::io::Result<()> {
    if let Some(file) = fs.locate(filename) {
        println!("File: {:#?}", file);
        if let HfsObjRef::FileRef(file) = file {
            if use_rsrc {
                let content = file.open_rsrc();
                println!("Content: {:#?}", content);
            } else {
                let mut s = String::new();
                file.open().read_to_string(&mut s)?;
                println!("Content: {:#?}", s);
            };
        }
    }
    Ok(())
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