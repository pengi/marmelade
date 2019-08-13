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


#[trap_handlers]
#[allow(non_snake_case)] // This function names comes from old Mac structs
impl ToolboxTrapHandler {
    #[trap(0xa036)]
    fn MoreMasters(&mut self, _core: &mut impl Core) -> Option<()> {
        println!("MoreMasters()");
        Some(())
    }
    
    #[trap(0xa86e)]
    fn InitGraf(&mut self, _core: &mut impl Core) -> Option<()> {
        println!("InitGraf()");
        Some(())
    }
    
    #[trap(0xa8fe)]
    fn InitFonts(&mut self, _core: &mut impl Core) -> Option<()> {
        println!("InitFonts()");
        Some(())
    }
    
    #[trap(0xa032)]
    fn FlushEvents(&mut self, _core: &mut impl Core) -> Option<()> {
        println!("FlushEvents()");
        Some(())
    }
    
    #[trap(0xa912)]
    fn InitWindows(&mut self, _core: &mut impl Core) -> Option<()> {
        println!("InitWindows()");
        Some(())
    }
    
    #[trap(0xa930)]
    fn InitMenus(&mut self, _core: &mut impl Core) -> Option<()> {
        println!("InitMenus()");
        Some(())
    }
    
    #[trap(0xa9cc)]
    fn TEInit(&mut self, _core: &mut impl Core) -> Option<()> {
        println!("TEInit()");
        Some(())
    }
    
    #[trap(0xa850)]
    fn InitCursor(&mut self, _core: &mut impl Core) -> Option<()> {
        println!("InitCursor()");
        Some(())
    }

    #[trap(0xa97b)]
    fn InitDialogs(&mut self, _core: &mut impl Core, resumeProc: u32) -> Option<()> {
        println!("InitDialogs(${:08x})", resumeProc);
        Some(())
    }

    #[trap(0xa063)]
    fn MaxApplZone(&mut self, core: &mut impl Core) -> Option<()> {
        core.dar()[0+0] = 0x01000000;
        Some(())
    }

    #[trap(0xa1ad)]
    fn Gestalt(&mut self, core: &mut impl Core) -> Option<()> {
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
        Some(())
    }

    #[trap(0xa260)]
    fn HFSDispatch(&mut self, _core: &mut impl Core) -> Option<()> {
        println!("HFSDispatch()");
        None
        
    }

    #[trap(0xa994)]
    fn CurResFile(&mut self, _core: &mut impl Core) -> Option<i16> {
        println!("CurResFile()");
        Some(1234)
    }

    #[trap(0xa346)]
    #[trap(0xa746)]
    fn GetTrapAddress(&mut self, core: &mut impl Core) -> Option<()> {
        let dar : &mut [u32; 16] = core.dar();
        // Input: D0 => trap number
        // Output: A0 => Handler address

        // TODO: This needs to be mocked, so it maps to an address space that triggers the trap anyway
        println!("GetTrapAddress({:02x})", dar[0+0]);

        dar[8+0] = 0xcafebabe;
        Some(())
    }

    #[trap(0xa9c9)]
    fn SysError(&mut self, core: &mut impl Core) -> Option<()> {
        println!("SysError code: {}", core.dar()[0] as i32);
        None
    }

    #[trap(0xa9f0)]
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

    #[trap(0xa9fd)]
    fn GetScrap(&mut self, _core: &mut impl Core, hDest: u32, theType: OSType, offset: i32) -> Option<i32> {
        println!("GetScrap(${:08x}, {:?}, {}) = -102", hDest, theType, offset);
        Some(-102i32)
    }
}