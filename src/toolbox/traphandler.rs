use super::{
    Toolbox
};

use crate::phy::{
    TrapHandler,
    TrapResult,
    Core,
    stackable::Stackable
};

use crate::types::OSType;

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
    fn Gestalt(&mut self, core: &mut impl Core, _pc: u32) -> TrapResult {
        let dar : &mut [u32; 16] = core.dar();
        // args
        let selector = OSType::from(dar[0+0]);

        // action
        println!("Gestalt({:?})", selector);

        // result
        let (code, _value): (i32, u32) = match &selector.0 {
            b"te  " => (0, 0),
            _ => (-5551, 0)
        };

        dar[0+0] = code as u32; // D0 = result code
        dar[8+0] = 0xcafebabe as u32; // A0 = Some global result
        TrapResult::Continue
    }

    fn HFSDispatch(&mut self, _core: &mut impl Core, _pc: u32) -> TrapResult {
        println!("HFSDispatch()");
        TrapResult::Halt
        
    }

    fn CurResFile(&mut self, core: &mut impl Core, _pc: u32) -> TrapResult {
        println!("CurResFile()");
        0x1234u16.stack_replace(core);
        TrapResult::Continue
    }

    fn GetTrapAddress(&mut self, core: &mut impl Core, _pc: u32) -> TrapResult {
        let dar : &mut [u32; 16] = core.dar();
        // Input: D0 => trap number
        // Output: A0 => Handler address

        // TODO: This needs to be mocked, so it maps to an address space that triggers the trap anyway
        println!("GetTrapAddress({:02x})", dar[0+0]);

        dar[8+0] = 0xcafebabe;
        TrapResult::Continue
    }

    fn SysError(&mut self, core: &mut impl Core, _pc: u32) -> TrapResult {
        println!("SysError code: {}", core.dar()[0] as i32);
        TrapResult::Halt
    }

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

    fn GetScrap(&mut self, _core: &mut impl Core, _pc: u32, hDest: u32, theType: OSType, offset: i32) -> i32 {
        println!("GetScrap(${:08x}, {:?}, {}) = -102", hDest, theType, offset);
        -102i32
    }

    fn invoke_GetScrap(&mut self, core: &mut impl Core, pc: u32) -> TrapResult {
        let arg_1 = Stackable::stack_pop(core);
        let arg_2 = Stackable::stack_pop(core);
        let arg_3 = Stackable::stack_pop(core);
        self.GetScrap(core, pc, arg_1, arg_2, arg_3).stack_replace(core);
        TrapResult::Continue
    }
}

impl TrapHandler for ToolboxTrapHandler {
    fn line_1010_emualtion(&mut self, core: &mut impl Core, ir: u16, pc: u32) -> TrapResult {
        match ir {
            0xa1ad => self.Gestalt(core, pc),
            0xa260 => self.HFSDispatch(core, pc),
            0xa346 => self.GetTrapAddress(core, pc),
            0xa746 => self.GetTrapAddress(core, pc),
            0xa994 => self.CurResFile(core, pc),
            0xa9c9 => self.SysError(core, pc),
            0xa9f0 => self.LoadSeg(core, pc),
            0xa9fd => self.invoke_GetScrap(core, pc),
            _ => {
                println!("Unimplemented trap {:04x} @ {:08x}", ir, pc);
                TrapResult::Halt
            }
        }
    }
}