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
    fn do_nothing(&mut self, _core: &mut impl Core, name: &str) -> TrapResult {
        println!("Calling: {}", name);
        TrapResult::Continue
    }

    fn InitDialogs(&mut self, _core: &mut impl Core, resumeProc: u32) -> Option<()> {
        println!("InitDialogs(${:08x})", resumeProc);
        Some(())
    }

    fn invoke_InitDialogs(&mut self, core: &mut impl Core) -> TrapResult {
        let arg_1 = Stackable::stack_pop(core);
        if let Some(result) = self.InitDialogs(core, arg_1) {
            result.stack_replace(core);
            TrapResult::Continue
        } else {
            TrapResult::Halt
        }
    }

    fn MaxApplZone(&mut self, core: &mut impl Core) -> TrapResult {
        core.dar()[0+0] = 0x01000000;
        TrapResult::Continue
    }

    fn Gestalt(&mut self, core: &mut impl Core) -> TrapResult {
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

    fn HFSDispatch(&mut self, _core: &mut impl Core) -> TrapResult {
        println!("HFSDispatch()");
        TrapResult::Halt
        
    }

    fn CurResFile(&mut self, _core: &mut impl Core) -> Option<i16> {
        println!("CurResFile()");
        Some(1234)
    }

    fn invoke_CurResFile(&mut self, core: &mut impl Core) -> TrapResult {
        if let Some(result) = self.CurResFile(core) {
            result.stack_replace(core);
            TrapResult::Continue
        } else {
            TrapResult::Halt
        }
    }

    fn GetTrapAddress(&mut self, core: &mut impl Core) -> TrapResult {
        let dar : &mut [u32; 16] = core.dar();
        // Input: D0 => trap number
        // Output: A0 => Handler address

        // TODO: This needs to be mocked, so it maps to an address space that triggers the trap anyway
        println!("GetTrapAddress({:02x})", dar[0+0]);

        dar[8+0] = 0xcafebabe;
        TrapResult::Continue
    }

    fn SysError(&mut self, core: &mut impl Core) -> TrapResult {
        println!("SysError code: {}", core.dar()[0] as i32);
        TrapResult::Halt
    }

    fn LoadSeg(&mut self, core: &mut impl Core, code_id: i16) -> Option<()> {
        if let Some(_address) = self.toolbox.segment_loader.borrow_mut().load(code_id) {
            // The segment is loaded, jump back to the jump table
            let pc = *core.pc();
            core.jump(pc - 6);
            Some(())
        } else {
            println!("Unknown segment {}", code_id);
            None
        }
    }

    fn invoke_LoadSeg(&mut self, core: &mut impl Core) -> TrapResult {
        let arg_1 = Stackable::stack_pop(core);
        if let Some(result) = self.LoadSeg(core, arg_1) {
            result.stack_replace(core);
            TrapResult::Continue
        } else {
            TrapResult::Halt
        }
    }

    fn GetScrap(&mut self, _core: &mut impl Core, hDest: u32, theType: OSType, offset: i32) -> Option<i32> {
        println!("GetScrap(${:08x}, {:?}, {}) = -102", hDest, theType, offset);
        Some(-102i32)
    }

    fn invoke_GetScrap(&mut self, core: &mut impl Core) -> TrapResult {
        let arg_1 = Stackable::stack_pop(core);
        let arg_2 = Stackable::stack_pop(core);
        let arg_3 = Stackable::stack_pop(core);
        if let Some(result) = self.GetScrap(core, arg_1, arg_2, arg_3) {
            result.stack_replace(core);
            TrapResult::Continue
        } else {
            TrapResult::Halt
        }
    }
}

impl TrapHandler for ToolboxTrapHandler {
    fn line_1010_emualtion(&mut self, core: &mut impl Core, ir: u16, _pc: u32) -> TrapResult {
        match ir {
            0xa036 => self.do_nothing(core, "MoreMasters"),
            0xa86e => self.do_nothing(core, "InitGraf"),
            0xa8fe => self.do_nothing(core, "InitFonts"),
            0xa032 => self.do_nothing(core, "FlushEvents"),
            0xa912 => self.do_nothing(core, "InitWindows"),
            0xa930 => self.do_nothing(core, "InitMenus"),
            0xa9cc => self.do_nothing(core, "TEInit"),
            0xa97b => self.invoke_InitDialogs(core),
            0xa850 => self.do_nothing(core, "InitCursor"),
            0xa063 => self.MaxApplZone(core),
            0xa1ad => self.Gestalt(core),
            0xa260 => self.HFSDispatch(core),
            0xa346 => self.GetTrapAddress(core),
            0xa746 => self.GetTrapAddress(core),
            0xa994 => self.invoke_CurResFile(core),
            0xa9c9 => self.SysError(core),
            0xa9f0 => self.invoke_LoadSeg(core),
            0xa9fd => self.invoke_GetScrap(core),
            _ => TrapResult::Unimplemented
        }
    }
}