use marmelade::filesys::hfs;
use std::fs;

fn main() {
    let mut file = fs::File::open("image.dmg").unwrap();
    let fs = hfs::HfsImage::from(&mut file);
    println!("{:#?}", fs);
}
