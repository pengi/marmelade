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
        let seg_no = core.pop_16();
        println!("LoadSeg({})", seg_no);
        core.jump(pc - 4);
        TrapResult::Continue
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