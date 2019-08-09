mod traphandler;
mod segment_loader;

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
            RAM,
            RcMem
        }
    },
    types::OSType
};
use std::rc::Rc;
use traphandler::ToolboxTrapHandler;
use segment_loader::SegmentLoader;
use std::borrow::BorrowMut;

type ToolboxPhy = Phy<MuxMem, ToolboxTrapHandler>;

pub struct Toolbox {
    img: HfsImage,
    rsrc: Rsrc,
    segment_loader: RcMem<SegmentLoader>

}

impl Toolbox {
    pub fn new(img: HfsImage, rsrc: Rsrc) -> std::io::Result<Rc<Toolbox>> {
        let toolbox = Rc::new(Toolbox {
            img,
            rsrc,
            segment_loader: RcMem::new(SegmentLoader::new(0x20000000, 8))
        });


        Ok(toolbox)
    }


    pub fn into_phy(toolbox: &Rc<Toolbox>) -> std::io::Result<ToolboxPhy> {
        let mut mem = MuxMem::new();

        // THe handlers is the main entry point to own the toolbox, since it's not owned back
        let handlers = ToolboxTrapHandler::new(toolbox.clone());

        // Jump table
        let jumptable_vec = toolbox.rsrc.open(OSType::from(b"CODE"), 0)?.to_vec();
        mem.add_prefix(Prefix::new(0x00001000, 20), Box::new(RAM::from(jumptable_vec)));

        // Segment loader
        let mut segment_loader = toolbox.segment_loader.borrow_mut();
        segment_loader.set_toolbox(Rc::downgrade(&toolbox));

        mem.add_prefix(
            segment_loader.get_prefix(),
            Box::new(toolbox.segment_loader.clone())
        );

        // Stack
        mem.add_prefix(Prefix::new(0xFFF00000, 12), Box::new(RAM::new(0x100000)));

        let mut phy = Phy::new(mem, handlers);

        phy.core.jump(0x1012); // Jump to first load entry in jump table

        Ok(phy)
    }
}