use super::{
    Toolbox
};

use crate::runner::{
    TrapHandler,
    TrapResult
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

impl TrapHandler for ToolboxTrapHandler {
    fn line_1010_emualtion(&mut self, ir: u16, pc: u32) -> TrapResult {
        println!("Toolbox trap {:04x} @ {:08x}", ir, pc);
        TrapResult::Halt
    }
}