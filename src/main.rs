#[macro_use]
extern crate clap;

use marmelade::filesys::hfs;
use marmelade::filesys::hfs::FileAdaptor;
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
    let fa = FileAdaptor::new(img);
    let fs = hfs::HfsImage::from(fa).unwrap();

    if let Some(file) = matches.value_of("file") {
        let file = fs.catalog.locate(file);
        println!("File: {:#?}", file);
    } else {
        // println!("Image: {:#?}", fs);
        fs.list_recursive(1, 0);
    }
}
