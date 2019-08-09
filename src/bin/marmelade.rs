#[macro_use]
extern crate clap;

use marmelade::{
    serialization::SerialAdaptor,
    filesys::hfs::HfsImage,
    filesys::rsrc::Rsrc,
    toolbox::Toolbox,
    tools::hexdump,
    types::OSType
};

use std::io::{
    ErrorKind
};
use std::fs;

fn main() -> std::io::Result<()> {
    let matches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Max Sikström <max@pengi.se>")
        (about: "Makes toasters fly - run old stuff on new machines")
        (@arg img: +required -i --image +takes_value "Image file")
        (@arg file: +required -f --file +takes_value "File to load")
    ).get_matches();

    let file_os_path = matches.value_of("img").ok_or(ErrorKind::from(ErrorKind::InvalidInput))?;
    let file_img_path = matches.value_of("file").ok_or(ErrorKind::from(ErrorKind::InvalidInput))?;
    let (fs, rsrc) = load_file(file_os_path, file_img_path)?;

    let jumptable = rsrc.open(OSType::from(b"CODE"), 0)?.to_vec();
    hexdump::hexdump(jumptable);

    let toolbox = Toolbox::new(fs, rsrc)?;
    let mut phy = Toolbox::into_phy(&toolbox)?;

    phy.run();

    Ok(())
}

fn load_file(file_os_path: &str, file_img_path: &str) -> std::io::Result<(HfsImage, Rsrc)> {
    let img_file = fs::File::open(file_os_path)?;
    let fs = HfsImage::from(SerialAdaptor::new(img_file))?;
    let rsrc_objref = fs.locate(file_img_path).ok_or(ErrorKind::from(ErrorKind::NotFound))?;
    let rsrc_fileref = rsrc_objref.to_file().ok_or(ErrorKind::from(ErrorKind::InvalidData))?;
    let rsrc = Rsrc::new(SerialAdaptor::new(rsrc_fileref.open_rsrc()))?;

    Ok((fs, rsrc))
}