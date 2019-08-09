mod traphandler;

use crate::{
    filesys::{
        hfs::HfsImage,
        rsrc::Rsrc
    },
    runner::{
        Runner,
        prefix::{
            Prefix
        },
        mem::{
            MuxMem,
            ROM
        }
    }
};

use std::rc::Rc;

use traphandler::ToolboxTrapHandler;

pub struct Toolbox {
    img: HfsImage,
    rsrc: Rsrc
}

impl Toolbox {
    pub fn new(img: HfsImage, rsrc: Rsrc) -> std::io::Result<Toolbox> {
        Ok(Toolbox {
            img,
            rsrc
        })
    }


    pub fn into_runner(self) -> (Rc<Toolbox>, Runner<MuxMem>) {
        let toolbox = Rc::new(self);
        let mut mem = MuxMem::new();

        let handlers = ToolboxTrapHandler::new(toolbox.clone());

        mem.add_prefix(Prefix::new(0x00001000, 20), Box::new(ROM::from(
            vec![0xa9, 0xf0]
        )));

        (toolbox, Runner::new(mem, Box::new(handlers)))
    }
}