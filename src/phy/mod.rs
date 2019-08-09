pub mod mem;
pub mod prefix;

use r68k_emu::{
    cpu::{
        ConfiguredCore,
        ProcessingState,
        Callbacks,
        Core,
        Cycles,
        Exception,
        Result
    },
    ram::{
        AddressBus
    },
    interrupts::AutoInterruptController
};

const START_ADDR : u32 = 0x1000;

pub type PhyCore<M> = ConfiguredCore<AutoInterruptController, M>;

pub struct Phy<M : AddressBus> {
    core: PhyCore<M>,
    callbacks: PhyCallbacks
}

impl<M : AddressBus> Phy<M> {
    pub fn new(membus: M, handlers: Box<dyn TrapHandler>) -> Phy<M> {
        let irq = AutoInterruptController::new();
        let core = PhyCore::new_with(START_ADDR, irq, membus);
        Phy {
            core,
            callbacks: PhyCallbacks::new(handlers)
        }
    }

    pub fn run(&mut self) -> () {
        for _ in 0..100 {
            self.print_core();
            self.core.execute_with_state(1, &mut self.callbacks);
            if self.core.processing_state == ProcessingState::Halted || self.core.processing_state == ProcessingState::Stopped {
                break;
            }
        }
        self.print_core();
    }

    fn print_core(&self) {
        println!("======================================");
        println!("PC:            {:08x}", self.core.pc);
        println!("SSP':          {:08x}", self.core.inactive_ssp);
        println!("USP':          {:08x}", self.core.inactive_usp);
        println!("IR:            {:08x}", self.core.ir);
        println!("D registers:   {:08x} {:08x} {:08x} {:08x}   {:08x} {:08x} {:08x} {:08x}",
            self.core.dar[0],
            self.core.dar[1],
            self.core.dar[2],
            self.core.dar[3],
            self.core.dar[4],
            self.core.dar[5],
            self.core.dar[6],
            self.core.dar[7]
        );
        println!("A registers:   {:08x} {:08x} {:08x} {:08x}   {:08x} {:08x} {:08x} {:08x}",
            self.core.dar[8+0],
            self.core.dar[8+1],
            self.core.dar[8+2],
            self.core.dar[8+3],
            self.core.dar[8+4],
            self.core.dar[8+5],
            self.core.dar[8+6],
            self.core.dar[8+7]
        );
        println!("S flag:        {:08x}", self.core.s_flag);
        println!("IRQ level:     {:08x}", self.core.irq_level);
        println!("INT mask:      {:08x}", self.core.int_mask);
        println!("X flag:        {:08x}", self.core.x_flag);
        println!("C flag:        {:08x}", self.core.c_flag);
        println!("V flag:        {:08x}", self.core.v_flag);
        println!("N flag:        {:08x}", self.core.n_flag);
        println!("Prefetch addr: {:08x}", self.core.prefetch_addr);
        println!("Prefetch data: {:08x}", self.core.prefetch_data);
        println!("not Z flag:    {:08x}", self.core.not_z_flag);
        println!("state:         {:?}", self.core.processing_state);
        println!("======================================");
    }
}

pub enum TrapResult {
    Continue,
    Halt
}

pub trait TrapHandler {
    fn line_1010_emualtion(&mut self, _ir: u16, _pc: u32) -> TrapResult {
        TrapResult::Continue
    }
}

struct PhyCallbacks {
    handler: Box<dyn TrapHandler>
}

impl PhyCallbacks {
    pub fn new(handler: Box<dyn TrapHandler>) -> PhyCallbacks {
        PhyCallbacks {
            handler
        }
    }
}

impl Callbacks for PhyCallbacks {
    fn exception_callback(&mut self, core: &mut impl Core, ex: Exception) -> Result<Cycles> {
        let action = match ex {
            Exception::UnimplementedInstruction(ir, pc, 10) => {
                self.handler.line_1010_emualtion(ir, pc)
            },
            _ => {
                println!("Unmatched handler: {:?}", ex);
                TrapResult::Continue
            }
        };
        match action {
            TrapResult::Continue => Err(ex),
            TrapResult::Halt => {
                core.stop_instruction_processing();
                Ok(Cycles(1))
            }
        }
    }
}