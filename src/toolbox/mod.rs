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
    },
    types::OSType
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
            segment_loader: RcMem::new(SegmentLoader::new(0x20000000, 8))
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
            segment_loader.get_prefix(),
            Box::new(toolbox.segment_loader.clone())
        );

        // Application RAM
        mem.add_prefix(Prefix::new(0x10e0_0000, 12), Box::new(RAM::new(0x0010_0000)));
        // Stack
        mem.add_prefix(Prefix::new(0x10f0_0000, 12), Box::new(RAM::new(0x0010_0000)));

        let mut phy = Phy::new(LogMem::new(mem, LOG_DATA), handlers);

        for i in 0..16 {
            phy.core.dar[i] = 0x01010101u32 * i as u32;
        }
        phy.core.dar[8+5] = 0x10e8_0000; // A5 - application base
        phy.core.dar[8+7] = 0x1100_0000; // A7 - stack pointer

        // Load jump table to RAM, at A5 + 32
        Self::load_jump_table(&mut phy.core, &toolbox.rsrc, 0x10e8_0000 + 32)?;

        // Start at first entry of jump table ()
        phy.core.jump(0x10e8_0000 + 32 + 16 + 2);

        // Push pointer, of some kind...
        phy.core.push_32(0xbaddecaf);

        Ok(phy)
    }

    fn load_jump_table(core: &mut impl Core, rsrc: &Rsrc, address: u32) -> std::io::Result<()> {
        let jumptable_vec = rsrc.open(OSType::from(b"CODE"), 0)?.to_vec();
        for (i, data) in jumptable_vec.iter().enumerate() {
            // write_program_byte or write_data_byte differs only in how it's logged currently
            // jump table is part code part data, and no memory protection is active
            core.write_program_byte(address + i as u32, *data as u32).unwrap();
        }
        Ok(())
    }
}