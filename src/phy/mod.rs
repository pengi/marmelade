pub mod mem;
pub mod prefix;
mod trace;

use r68k_emu::{
    cpu::{
        ConfiguredCore,
        ProcessingState,
        Callbacks,
        Cycles,
        Exception,
        Result
    },
    interrupts::AutoInterruptController
};

pub use r68k_emu::{
    cpu::Core,
    ram::AddressBus
};

type PhyCore<M> = ConfiguredCore<AutoInterruptController, M>;

pub struct Phy<M : AddressBus, T : TrapHandler> {
    pub core: PhyCore<M>,
    callbacks: PhyCallbacks<T>
}

impl<M : AddressBus, T : TrapHandler> Phy<M, T> {
    pub fn new(membus: M, handlers: T) -> Phy<M, T> {
        let irq = AutoInterruptController::new();
        let core = PhyCore::new_with(0, irq, membus);
        Phy {
            core,
            callbacks: PhyCallbacks::new(handlers)
        }
    }

    pub fn run(&mut self) -> () {
        trace::print_core_header(&self);
        for _ in 0..100 {
            trace::print_core_line(&self);
            self.core.execute_with_state(1, &mut self.callbacks);
            if self.core.processing_state == ProcessingState::Halted || self.core.processing_state == ProcessingState::Stopped {
                break;
            }
        }
        trace::print_core(&self);
    }
}

pub enum TrapResult {
    Exception, // Run as exception handler
    Continue,  // Continue as normal operation
    Halt,      // Halt CPU
}

pub trait TrapHandler {
    fn line_1010_emualtion(&mut self, _core: &mut impl Core, _ir: u16, _pc: u32) -> TrapResult {
        TrapResult::Continue
    }
}

struct PhyCallbacks<T : TrapHandler> {
    handler: T
}

impl<T : TrapHandler> PhyCallbacks<T> {
    pub fn new(handler: T) -> PhyCallbacks<T> {
        PhyCallbacks {
            handler
        }
    }
}

impl<T : TrapHandler> Callbacks for PhyCallbacks<T> {
    fn exception_callback(&mut self, core: &mut impl Core, ex: Exception) -> Result<Cycles> {
        let action = match ex {
            Exception::UnimplementedInstruction(ir, pc, 10) => {
                self.handler.line_1010_emualtion(core, ir, pc)
            },
            _ => {
                println!("Unmatched handler: {:?}", ex);
                TrapResult::Halt
            }
        };
        match action {
            TrapResult::Exception => Err(ex),
            TrapResult::Continue => Ok(Cycles(1)),
            TrapResult::Halt => {
                core.stop_instruction_processing();
                Ok(Cycles(1))
            }
        }
    }
}