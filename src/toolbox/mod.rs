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
            RAM
        }
    },
    types::OSType
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


    pub fn into_phy(self) -> std::io::Result<(Rc<Toolbox>, ToolboxPhy)> {
        let toolbox = Rc::new(self);
        let mut mem = MuxMem::new();

        let handlers = ToolboxTrapHandler::new(toolbox.clone());

        // Jump table
        let jumptable_vec = toolbox.rsrc.open(OSType::from(b"CODE"), 0)?.to_vec();
        mem.add_prefix(Prefix::new(0x00001000, 20), Box::new(RAM::from(jumptable_vec)));

        // Stack
        mem.add_prefix(Prefix::new(0xFFF00000, 12), Box::new(RAM::new(0x100000)));

        let mut phy = Phy::new(mem, handlers);

        phy.core.jump(0x1012); // Jump to first load entry in jump table

        Ok((toolbox, phy))
    }
}