use marmelade::filesys::hfs;
use marmelade::filesys::hfs::fileadaptor::FileAdaptor;
use std::fs;

fn main() {
    let mut file = fs::File::open("image.dmg").unwrap();
    let mut fa = FileAdaptor::new(&mut file);
    let fs = hfs::HfsImage::from(&mut fa);
    println!("{:#?}", fs);
}
