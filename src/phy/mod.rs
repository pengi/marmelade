pub mod mem;
pub mod prefix;

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

const START_ADDR : u32 = 0x1000;

type PhyCore<M> = ConfiguredCore<AutoInterruptController, M>;

pub struct Phy<M : AddressBus, T : TrapHandler> {
    pub core: PhyCore<M>,
    callbacks: PhyCallbacks<T>
}

impl<M : AddressBus, T : TrapHandler> Phy<M, T> {
    pub fn new(membus: M, handlers: T) -> Phy<M, T> {
        let irq = AutoInterruptController::new();
        let core = PhyCore::new_with(START_ADDR, irq, membus);
        Phy {
            core,
            callbacks: PhyCallbacks::new(handlers)
        }
    }

    pub fn run(&mut self) -> () {
        self.print_core_header();
        for _ in 0..100 {
            self.print_core_line();
            self.core.execute_with_state(1, &mut self.callbacks);
            if self.core.processing_state == ProcessingState::Halted || self.core.processing_state == ProcessingState::Stopped {
                break;
            }
        }
        self.print_core();
    }

    fn print_core_header(&self) {
        println!(
            "PC...... IR.. SSP'.... USP'....   D0...... D1...... D2...... D3...... D4...... D5...... D6...... D7......   A0...... A1...... A2...... A3...... A4...... A5...... A6...... A7......   Sflag... I# INT.mask Xflag... Cflag... Vflag... Nflag... Z'flag.. state");
    }
    fn print_core_line(&self) {
        let c = &self.core;
        println!(
            "{:08x} {:04x} {:08x} {:08x}   {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x}   {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x}   {:08x} {:02x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:?}",
            c.pc, c.ir, c.inactive_ssp, c.inactive_usp,
            c.dar[0], c.dar[1], c.dar[2], c.dar[3], c.dar[4], c.dar[5], c.dar[6], c.dar[7],
            c.dar[8], c.dar[9], c.dar[10], c.dar[11], c.dar[12], c.dar[13], c.dar[14], c.dar[15],
            c.s_flag, c.irq_level, c.int_mask, c.x_flag, c.c_flag, c.v_flag, c.n_flag, c.not_z_flag, c.processing_state
        );
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