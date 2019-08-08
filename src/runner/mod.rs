pub mod addressbus;
pub mod rom;
pub mod prefix;

use r68k_emu::{
    cpu::{
        ConfiguredCore,
        ProcessingState,
        Callbacks,
        Exception,
        Cycles,
        Core,
        Result
    },
    interrupts::AutoInterruptController
};

use prefix::Prefix;

use crate::filesys::{hfs::HfsImage, rsrc::Rsrc};

const START_ADDR : u32 = 0x1000;

pub type RunnerCore = ConfiguredCore<AutoInterruptController, addressbus::MuxAddressBus>;

pub struct Runner {
    core: RunnerCore
}

impl Runner {
    pub fn new(_img: &HfsImage, _rsrc: &Rsrc) -> std::io::Result<Runner> {
        let irq = AutoInterruptController::new();
        let addr_bus = {
            let mut bus = addressbus::MuxAddressBus::new();
            bus.add_prefix(Prefix::new(0x00001000, 20), Box::from(rom::ROM::from(
                vec![0x3F, 0x3C, 0x00, 0x01, 0xA9, 0xF0] // push #1, call LoadSeg
            )));
            bus
        };
        let core = RunnerCore::new_with(START_ADDR, irq, addr_bus);
        Ok(Runner { core })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        for _ in 0..100 {
            self.print_core();
            self.core.execute_with_state(1, &mut RunnerCallbacks);
            if self.core.processing_state == ProcessingState::Halted || self.core.processing_state == ProcessingState::Stopped {
                break;
            }
        }
        self.print_core();
        Ok(())
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

struct RunnerCallbacks;
impl Callbacks for RunnerCallbacks {
    fn exception_callback(&mut self, core: &mut impl Core, ex: Exception) -> Result<Cycles> {
        match ex {
            Exception::UnimplementedInstruction(ir, pc, _) if (ir&0xf000) == 0xa000 => {
                println!("Toolbox trap {:04x} at {:08x}", ir, pc);
                core.stop_instruction_processing();
                Ok(Cycles(10))
            },
            _ => Err(ex)
        }
    }
}