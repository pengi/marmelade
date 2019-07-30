use marmelade::filesys::hfs;
use marmelade::filesys::hfs::fileadaptor::FileAdaptor;
use std::fs;

fn main() {
    let mut file = fs::File::open("ref/refdisk.dmg").unwrap();
    let mut fa = FileAdaptor::new(&mut file);
    let fs = hfs::HfsImage::from(&mut fa).unwrap();

    println!("{:#?}", fs);

    fs.list_recursive(1, 0);
    let file = fs.catalog.locate("SimpleText");
    println!("File: {:#?}", file);
    // fs.list_files().unwrap();
}
