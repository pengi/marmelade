#[macro_use]
extern crate clap;

use marmelade::filesys::hfs;
use marmelade::filesys::hfs::DiskAdaptor;
use marmelade::filesys::hfs::HfsDirIter;
use std::fs;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Max Sikström <max@pengi.se>")
        (about: "Makes toasters fly - run old stuff on new machines")
        (@arg img: +required -i --image +takes_value "Image file")
        (@arg file: -f --file +takes_value "File to read")
    ).get_matches();

    let img = fs::File::open(matches.value_of("img").unwrap()).unwrap();
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
        if obj.is_dir() {
            println!("{}{:?}:", indstr, obj.get_name());
            print_files(obj.open_dir().unwrap(), indent+1);
        }
        if obj.is_file() {
            println!("{}{:?}", indstr, obj.get_name());
        }
    }
}