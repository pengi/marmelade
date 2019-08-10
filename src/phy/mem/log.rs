
use r68k_emu::ram::{
    AddressBus,
    AddressSpace,
    USER_DATA,
    USER_PROGRAM,
    SUPERVISOR_DATA,
    SUPERVISOR_PROGRAM,
};

pub const LOG_DATA : u32 = 0x00000001;
pub const LOG_PROGRAM : u32 = 0x00000002;

pub struct LogMem<M:AddressBus> {
    log_level: u32,
    child: M
}

impl<M:AddressBus> AddressBus for LogMem<M> {
    fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32 {
        let value = self.child.read_byte(address_space, address);
        if self.should_log(address_space) {
            println!("{:02x} = mem[{:08x}]", value, address);
        }
        value
    }
    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
        let value = self.child.read_word(address_space, address);
        if self.should_log(address_space) {
            println!("{:04x} = mem[{:08x}]", value, address);
        }
        value
    }
    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
        let value = self.child.read_long(address_space, address);
        if self.should_log(address_space) {
            println!("{:08x} = mem[{:08x}]", value, address);
        }
        value
    }
    fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        if self.should_log(address_space) {
            println!("mem[{:08x}] = {:02x}", address, value);
        }
        self.child.write_byte(address_space, address, value);
    }
    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        if self.should_log(address_space) {
            println!("mem[{:08x}] = {:04x}", address, value);
        }
        self.child.write_word(address_space, address, value);
    }
    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        if self.should_log(address_space) {
            println!("mem[{:08x}] = {:08x}", address, value);
        }
        self.child.write_long(address_space, address, value);
    }
}

impl<M:AddressBus> LogMem<M> {
    pub fn new(child: M, log_level: u32) -> LogMem<M> {
        LogMem { log_level, child }
    }

    fn should_log(&self, address_space: AddressSpace) -> bool {
        match address_space {
            USER_DATA => (self.log_level & LOG_DATA) != 0,
            USER_PROGRAM => (self.log_level & LOG_PROGRAM) != 0,
            SUPERVISOR_DATA => (self.log_level & LOG_DATA) != 0,
            SUPERVISOR_PROGRAM => (self.log_level & LOG_PROGRAM) != 0,
        }
    }
}