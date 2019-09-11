use super::CPUBus;
use r68k_emu::{cpu, cpu::{Core, Callbacks, Exception, Cycles}};

use std::rc::Rc;
use std::cell::{RefCell, Ref};

use super::trace;

pub struct CPUCallbacksInterface {
    peripheral: Rc<RefCell<CPUBus>>
}

impl CPUCallbacksInterface {
    pub fn new(peripheral: &Rc<RefCell<CPUBus>>) -> CPUCallbacksInterface {
        CPUCallbacksInterface {
            peripheral: Rc::clone(peripheral)
        }
    }
}

impl Callbacks for CPUCallbacksInterface {
    fn exception_callback(&mut self, core: &mut impl Core, ex: Exception) -> cpu::Result<Cycles> {
        let p: Ref<_> = self.peripheral.borrow();
        trace::print_exception("Ex", ex);
        
        let action = match ex {
            Exception::UnimplementedInstruction(ir, pc, 10) => {
                p.line_1010_emualtion(ir, pc)
            },
            _ => None
        };
        if let Some(_) = action {
            Ok(Cycles(1))
        } else {
            core.stop_instruction_processing();
            Ok(Cycles(1))
        }
    }
}

