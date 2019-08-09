use super::{
    Toolbox
};

use crate::phy::{
    TrapHandler,
    TrapResult,
    Core
};

use std::rc::Rc;

pub struct ToolboxTrapHandler {
    toolbox: Rc<Toolbox>
}

impl ToolboxTrapHandler {
    pub fn new(toolbox: Rc<Toolbox>) -> ToolboxTrapHandler {
        ToolboxTrapHandler {
            toolbox
        }
    }
}


#[allow(non_snake_case)] // This function names comes from old Mac structs
impl ToolboxTrapHandler {
    fn LoadSeg(&mut self, core: &mut impl Core, pc: u32) -> TrapResult {
        let code_id = core.pop_16() as i16;
        if let Some(address) = self.toolbox.segment_loader.borrow_mut().load(code_id) {
            // Read metadata from jump table
            let offset = core.read_data_word(pc - 6).unwrap(); // offset field

            // Update jump table to jump instruction
            core.write_data_word(pc - 6, (code_id as u16) as u32).unwrap(); // Store section id
            core.write_data_word(pc - 4, 0x4ef9).unwrap(); // jump absolute long
            core.write_data_long(pc - 2, offset as u32 + address).unwrap();
            core.jump(pc - 4);
            TrapResult::Continue
        } else {
            println!("Unknown segment {}", code_id);
            TrapResult::Halt
        }
    }
}

impl TrapHandler for ToolboxTrapHandler {
    fn line_1010_emualtion(&mut self, core: &mut impl Core, ir: u16, pc: u32) -> TrapResult {
        match ir {
            0xa9f0 => self.LoadSeg(core, pc),
            _ => {
                println!("Unimplemented trap {:04x} @ {:08x}", ir, pc);
                TrapResult::Halt
            }
        }
    }
}