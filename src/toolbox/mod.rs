mod traphandler;

use crate::{
    filesys::{
        hfs::HfsImage,
        rsrc::Rsrc
    },
    phy::{
        Phy,
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

type ToolboxPhy = Phy<MuxMem, ToolboxTrapHandler>;

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


    pub fn into_phy(self) -> (Rc<Toolbox>, ToolboxPhy) {
        let toolbox = Rc::new(self);
        let mut mem = MuxMem::new();

        let handlers = ToolboxTrapHandler::new(toolbox.clone());

        mem.add_prefix(Prefix::new(0x00001000, 20), Box::new(ROM::from(
            vec![0x3f, 0x3c, 0x00, 0x01, 0xa9, 0xf0]
        )));

        (toolbox, Phy::new(mem, handlers))
    }
}