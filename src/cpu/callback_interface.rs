use super::CPUBus;
use r68k_emu::{cpu, cpu::{Core, Callbacks, Exception, Cycles}};

use std::rc::Rc;
use std::cell::{RefCell, Ref};

use super::trace;

pub struct CPUCallbacksInterface<'cpu> {
    peripheral: &'cpu mut cpu
}

impl<'cpu> CPUCallbacksInterface<'cpu> {
    pub fn new(peripheral: &Rc<RefCell<CPUBus>>) -> CPUCallbacksInterface {
        CPUCallbacksInterface {
            cpu: CPU
        }
    }
}

impl<'cpu> Callbacks for CPUCallbacksInterface<'cpu> {
    fn exception_callback(&mut self, core: &mut impl Core, ex: Exception) -> cpu::Result<Cycles> {
        let p: Ref<_> = self.peripheral.borrow();
        let cpu: 
        trace::print_exception("Ex", ex);
        
        let action = match ex {
            Exception::UnimplementedInstruction(ir, pc, 10) => {
                p.line_1010_emualtion(cpu, ir, pc)
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

