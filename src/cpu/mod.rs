mod bus;
mod peripheral;
mod address_interface;
mod callback_interface;

mod trace;

mod stackable;
pub use stackable::Stackable;

mod prefix;
pub use prefix::AddressRange;

pub use peripheral::CPUPeripheral;
use bus::CPUBus;

use address_interface::CPUAddressInterface;
use callback_interface::CPUCallbacksInterface;

use std::rc::Rc;
use std::cell::{RefCell, RefMut};

use r68k_emu::{
    cpu::{
        ConfiguredCore,
        ProcessingState
    },
    interrupts::AutoInterruptController,
};

pub struct CPU {
    core: ConfiguredCore<AutoInterruptController, CPUAddressInterface>,
    callback_interface: CPUCallbacksInterface,
    bus: Rc<RefCell<CPUBus>>
}

impl CPU {
    pub fn new() -> CPU {
        let bus = Rc::new(RefCell::new(CPUBus::new()));
        let address_interface = CPUAddressInterface::new(&bus);
        let callback_interface = CPUCallbacksInterface::new(&bus);
        
        let irq = AutoInterruptController::new();
        
        CPU {
            core: ConfiguredCore::new_with(0, irq, address_interface),
            callback_interface,
            bus
        }
    }
    
    pub fn attach(&mut self, peripheral: Box<dyn CPUPeripheral>) {
        let mut bus: RefMut<_> = self.bus.borrow_mut();
        bus.attach(peripheral)
    }
    
    pub fn run(&mut self) {
        trace::print_core_header(&self);
        loop {
            trace::print_core_line(&self);
            self.core.execute_with_state(1, &mut self.callback_interface);
            if self.core.processing_state == ProcessingState::Halted || self.core.processing_state == ProcessingState::Stopped {
                break;
            }
        }
        trace::print_core(&self);
    }
}