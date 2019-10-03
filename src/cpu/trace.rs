use r68k_emu::{
    ram::AddressBus,
    ram::SUPERVISOR_PROGRAM,
    cpu::{Core, Exception}
};

use r68k_tools::{
    PC,
    disassembler::disassemble,
    memory::Memory,
};

struct PhyMemory<'phy> {
    core: &'phy dyn Core
}

impl<'phy> PhyMemory<'phy> {
    pub fn new(core: &'phy dyn Core) -> PhyMemory<'phy> {
        PhyMemory { core: core }
    }
}

impl<'phy> Memory for PhyMemory<'phy> {
    fn offset(&self) -> u32 {
        *self.core.pc()
    }
    fn data(&self) -> &[u8] {
        unimplemented!();
    }
    fn read_word(&self, pc: PC) -> u16 {
        self.core.read_program_word(pc.0).unwrap() as u16
    }
    fn read_byte(&self, pc: PC) -> u8 {
        self.core.read_program_byte(pc.0).unwrap() as u8
    }
    fn write_byte(&mut self, _pc: PC, _byte: u8) -> PC {
        unimplemented!();
    }
    fn write_word(&mut self, _pc: PC, _word: u16) -> PC {
        unimplemented!();
    }
    fn write_vec(&mut self, _pc: PC, _bytes: Vec<u8>) -> PC {
        unimplemented!();
    }
}

pub fn print_core_header(_tbx : &dyn Core) {
    println!(
        "        PC...... | IR.. | D0...... D1...... D2...... D3...... D4...... D5...... D6...... D7...... | A0...... A1...... A2...... A3...... A4...... A5...... A6...... A7...... | Next Instruction");
}

pub fn print_core_line(c : &dyn Core) {
    print!(
        "        {:08x} | {:04x} | {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} | {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} | ",
        c.pc(), c.ir(),
        c.dar()[0], c.dar()[1], c.dar()[2], c.dar()[3], c.dar()[4], c.dar()[5], c.dar()[6], c.dar()[7],
        c.dar()[8], c.dar()[9], c.dar()[10], c.dar()[11], c.dar()[12], c.dar()[13], c.dar()[14], c.dar()[15],
    );

    let mem = PhyMemory::new(c);
    print!("${:04x}  ", mem.read_word(PC(*c.pc())));
    if let Ok((_, inst)) = disassemble(PC(*c.pc()), &mem) {
        let mut inst = format!("{}", inst);
        if let Some(delim) = inst.find("\t") {
            let arg = inst.split_off(delim);
            println!("{:10} {}",
                inst.trim().to_lowercase(),
                arg.trim().replace(",", ", ")
            );
        } else {
            println!("{}", inst.trim());
        }
    } else {
        println!("");
    }
}

pub fn print_core(c : &dyn Core) {
    println!("======================================");
    println!("PC:            {:08x}", c.pc());
    println!("SSP':          {:08x}", c.inactive_ssp());
    println!("USP':          {:08x}", c.inactive_usp());
    println!("IR:            {:08x}", c.ir());
    println!("D registers:   {:08x} {:08x} {:08x} {:08x}   {:08x} {:08x} {:08x} {:08x}",
        c.dar()[0],
        c.dar()[1],
        c.dar()[2],
        c.dar()[3],
        c.dar()[4],
        c.dar()[5],
        c.dar()[6],
        c.dar()[7]
    );
    println!("A registers:   {:08x} {:08x} {:08x} {:08x}   {:08x} {:08x} {:08x} {:08x}",
        c.dar()[8+0],
        c.dar()[8+1],
        c.dar()[8+2],
        c.dar()[8+3],
        c.dar()[8+4],
        c.dar()[8+5],
        c.dar()[8+6],
        c.dar()[8+7]
    );
    println!("S flag:        {:08x}", c.s_flag());
    // println!("IRQ level:     {:08x}", c.irq_level());
    // println!("INT mask:      {:08x}", c.int_mask());
    println!("X flag:        {:08x}", c.x_flag());
    println!("C flag:        {:08x}", c.c_flag());
    println!("V flag:        {:08x}", c.v_flag());
    println!("N flag:        {:08x}", c.n_flag());
    // println!("Prefetch addr: {:08x}", c.prefetch_addr());
    // println!("Prefetch data: {:08x}", c.prefetch_data());
    println!("not Z flag:    {:08x}", c.not_z_flag());
    // println!("state:         {:?}", c.processing_state());
    println!("======================================");
}

pub fn print_exception(prefix: &str, ex: Exception) {
    match ex {
        Exception::AddressError {address, access_type, processing_state, address_space} => {
            println!("{}: AddressError", prefix);
            println!("    address:          ${:08x}", address);
            println!("    access type:      {:?}", access_type);
            println!("    processing state: {:?}", processing_state);
            println!("    address space:    {:?}", address_space);
        },
        Exception::IllegalInstruction(ir, pc) => {
            println!("{}: IllegalInstruction", prefix);
            println!("    IR:               ${:04x}", ir);
            println!("    PC:               ${:08x}", pc);
        },
        Exception::Trap(no, cycles) => {
            println!("{}: Trap", prefix);
            println!("    no:               ${:02x}", no);
            println!("    cycles:           {}", cycles);
        },
        Exception::PrivilegeViolation(ir, pc) => {
            println!("{}: PrivilegeViolation", prefix);
            println!("    IR:               ${:04x}", ir);
            println!("    PC:               ${:08x}", pc);
        },
        Exception::UnimplementedInstruction(ir, pc, vector) => {
            println!("{}: UnimplementedInstruction", prefix);
            println!("    IR:               ${:04x}", ir);
            println!("    PC:               ${:08x}", pc);
            println!("    vector:           ${:02x}", vector);
        },
        Exception::Interrupt(irq, vector) => {
            println!("{}: Interrupt", prefix);
            println!("    IRQ:              ${:02x}", irq);
            println!("    vector:           ${:02x}", vector);
        }
    };
}