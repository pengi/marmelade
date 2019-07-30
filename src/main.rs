use marmelade::filesys::hfs;
use marmelade::filesys::hfs::fileadaptor::FileAdaptor;
use std::fs;

fn main() {
    let file = fs::File::open("ref/refdisk.dmg").unwrap();
    let fa = FileAdaptor::new(file);
    let fs = hfs::HfsImage::from(fa).unwrap();

    println!("{:#?}", fs);

    fs.list_recursive(1, 0);
    let file = fs.catalog.locate("SimpleText");
    println!("File: {:#?}", file);
    // fs.list_files().unwrap();
}
