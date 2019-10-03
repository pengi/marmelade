mod bus;
mod peripheral;
mod address_interface;

mod trace;

mod stackable;
pub use stackable::Stackable;

mod prefix;
pub use prefix::AddressRange;

pub use peripheral::CPUPeripheral;
use bus::CPUBus;

use address_interface::CPUAddressInterface;

use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};

use r68k_emu::{
    cpu,
    cpu::{
        ConfiguredCore,
        ProcessingState,
        Callbacks,
        Exception,
        Cycles,
        Core
    },
    interrupts::AutoInterruptController,
};

pub type CPUCore = ConfiguredCore<AutoInterruptController, CPUAddressInterface>;

pub struct CPU {
    core: Option<CPUCore>,
    bus: Rc<RefCell<CPUBus>>
}

impl CPU {
    pub fn new() -> CPU {
        let bus = Rc::new(RefCell::new(CPUBus::new()));
        let address_interface = CPUAddressInterface::new(&bus);
        
        let irq = AutoInterruptController::new();
        
        CPU {
            core: Some(ConfiguredCore::new_with(0, irq, address_interface)),
            bus
        }
    }
    
    pub fn attach(&mut self, peripheral: Box<dyn CPUPeripheral>) {
        let mut bus: RefMut<_> = self.bus.borrow_mut();
        bus.attach(peripheral)
    }
    
    pub fn run(&mut self) {
        if let Some(core) = self.core {
            self.core = None;
            trace::print_core_header(&core);
            loop {
                trace::print_core_line(&core);
                core.execute_with_state(1, self);
                if core.processing_state == ProcessingState::Halted || core.processing_state == ProcessingState::Stopped {
                    break;
                }
            }
            trace::print_core(&core);
            self.core = Some(core);
        } else {
            println!("CPU Core is occupied");
        }
    }
}

impl Callbacks for CPU {
    fn exception_callback(&mut self, core: &mut impl Core, ex: Exception) -> cpu::Result<Cycles> {
        let p: Ref<_> = self.bus.borrow();
        trace::print_exception("Ex", ex);
        
        let action = match ex {
            Exception::UnimplementedInstruction(ir, pc, 10) => {
                p.line_1010_emualtion(self, core, ir, pc)
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