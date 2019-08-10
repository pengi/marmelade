#[macro_use]
extern crate clap;

use marmelade::{
    serialization::{
        SerialAdaptor,
        SerialReadStorage,
        SerialRead
    },
    filesys::hfs::{
        self,
        HfsObjRef,
        HfsDirIter
    },
    filesys::rsrc::{
        Rsrc
    },
    types::{
        OSType
    },
    tools::hexdump
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
        (@arg type: -T --type +takes_value "hexdump given rsrc type")
        (@arg id: -I --id +takes_value "hexdump given rsrc id")
    ).get_matches();

    let imgfile = matches.value_of("img").unwrap();
    let img = fs::File::open(imgfile).unwrap();
    let fa = SerialAdaptor::new(img);
    let fs = hfs::HfsImage::from(fa).unwrap();

    if let Some(file) = matches.value_of("file") {
        let use_rsrc = matches.occurrences_of("rsrc") > 0;

        let mut type_id = None;

        if let Some(rsrc_type) = matches.value_of("type") {
            if let Some(rsrc_id) = matches.value_of("id") {
                type_id = Some((
                    OSType::from(rsrc_type.as_bytes()),
                    i16::from_str_radix(rsrc_id, 10).unwrap()
                ));
            }
        }

        if let Err(err) = open_file(&fs, file, use_rsrc, type_id) {
            eprintln!("Error: {}", err);
        }
    } else {
        let prefix = String::from("");
        print_files(fs.open_root(), &prefix);
    }
}

fn open_file(fs: &hfs::HfsImage, filename: &str, use_rsrc: bool, type_id: Option<(OSType, i16)>) -> std::io::Result<()> {
    if let Some(file) = fs.locate(filename) {
        println!("File: {:#?}", file);
        if let HfsObjRef::FileRef(file) = file {
            if use_rsrc {
                let content = file.open_rsrc();
                let rsrc_adaptor = SerialAdaptor::new(content);
                let rsrc = Rsrc::new(rsrc_adaptor)?;
                println!("Content: {:#?}", rsrc);
                if let Some((rsrc_type, rsrc_id)) = type_id {
                    if let Ok(storage) = rsrc.open(rsrc_type, rsrc_id) {
                        hexdump::hexdump(&storage.to_vec());
                    }
                } else {
                    if let Ok(mut storage) = rsrc.open(OSType::from(b"ICN#"), 128) {
                        println!("Icon:");
                        icon_render(&mut storage);
                    }
                }
            } else {
                let mut s = String::new();
                file.open().read_to_string(&mut s)?;
                println!("Content: {:#?}", s);
            };
        }
    }
    Ok(())
}


fn print_files(dir: HfsDirIter, prefix: &String) {
    for obj in dir {
        match obj {
            HfsObjRef::FileRef(file) => {
                let (data_size, rsrc_size) = file.get_size();
                println!("{}:{} (size: {}/{})", prefix, file.get_name(), data_size, rsrc_size);
            },
            HfsObjRef::DirRef(dir) => {
                let sub_prefix = format!("{}:{}", prefix, dir.get_name());
                println!("{} (dir)", sub_prefix);
                print_files(dir.open(), &sub_prefix);
            }
        }
    }
}


fn icon_render(rdr: &mut SerialReadStorage) {
    let mut bytes = [0 as u8; 256];
    for i in 0..256 {
        bytes[i] = u8::read(rdr).unwrap();
    }

    for y in 0..32 {
        for x in 0..32 {
            let pxlidx = y*32 + x;
            let pxlbyte = pxlidx / 8;
            let pxlbit = 1<<(7-(pxlidx%8));
            let mask = bytes[pxlbyte+128]&pxlbit != 0;
            let col = bytes[pxlbyte]&pxlbit != 0;
            let chr = if !mask {'.'} else if col {'#'} else {' '};
            print!("{}{}", chr, chr);
        }
        println!("");
    }
}