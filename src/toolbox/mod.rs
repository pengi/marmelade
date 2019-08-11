mod traphandler;
mod segment_loader;

// The toolbox emulates the functionality of Macintosh Toolbox

// Memory map:
// 00000xxx - Global variables (not yet implemented)
// 00001xxx - Jump table
// 10exxxxx - Application globals (A5 @ 10e80000)
// 10fxxxxx - Stack
// 20xxxxxx - Segment loade
// 8xxxxxxx - Dynamic RAM (not yet implemented)


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
            LogMem,
            RAM,
            RcMem,
            log::{
                LOG_DATA
            }
        }
    }
};
use std::rc::Rc;
use traphandler::ToolboxTrapHandler;
use segment_loader::SegmentLoader;

pub use r68k_emu::cpu::Core;

type ToolboxPhy = Phy<LogMem<MuxMem>, ToolboxTrapHandler>;

pub struct Toolbox {
    _img: HfsImage,
    rsrc: Rsrc,
    segment_loader: RcMem<SegmentLoader>

}

impl Toolbox {
    pub fn new(img: HfsImage, rsrc: Rsrc) -> std::io::Result<Rc<Toolbox>> {
        let toolbox = Rc::new(Toolbox {
            _img: img,
            rsrc,
            segment_loader: RcMem::new(SegmentLoader::new())
        });

        Ok(toolbox)
    }


    pub fn into_phy(toolbox: &Rc<Toolbox>) -> std::io::Result<ToolboxPhy> {
        let mut mem = MuxMem::new();

        // THe handlers is the main entry point to own the toolbox, since it's not owned back
        let handlers = ToolboxTrapHandler::new(toolbox.clone());

        // Segment loader
        let mut segment_loader = toolbox.segment_loader.borrow_mut();
        segment_loader.set_toolbox(Rc::downgrade(&toolbox));

        mem.add_prefix(
            segment_loader.set_prefix(0x20000000, 8),
            Box::new(toolbox.segment_loader.clone())
        );

        // Globals RAM
        mem.add_prefix(Prefix::new(0x0000_0000, 20), Box::new(RAM::from(vec![0x00u8; 0x1000])));

        // Stack
        mem.add_prefix(Prefix::new(0x10f0_0000, 12), Box::new(RAM::new(0x0010_0000)));

        // Application RAM needs to preceed the jump table, since relative to A5
        mem.add_prefix(Prefix::new(0x1ff0_0000, 12), Box::new(RAM::new(0x0010_0000)));

        let mut phy = Phy::new(LogMem::new(mem, LOG_DATA), handlers);

        phy.core.dar = [
            0xd0d0_d0d0, // D0
            0xd1d1_d1d1, // D1
            0xd2d2_d2d2, // D2
            0x1ff0_0000, // D3
            0xd4d4_d4d4, // D4
            0xd5d5_d5d5, // D5
            0xd6d6_d6d6, // D6
            0xd7d7_d7d7, // D7
            0x0000_0f00, // A0 ? - initial address to finder info?
            0xa1a1_a1a1, // A1
            0xa2a2_a2a2, // A2
            0xa3a3_a3a3, // A3
            0xa4a4_a4a4, // A4
            segment_loader.get_a5(), // A5 - application base (jump table - 32B)
            0xa6a6_a6a6, // A6 - stack frame
            0x1100_0000  // A7 - stack pointer
        ];


        // Start at first entry of jump table 18(A5)
        phy.core.jump(segment_loader.get_start());

        // Push pointer, of some kind...
        phy.core.push_32(0xcafebabe);

        Ok(phy)
    }
}