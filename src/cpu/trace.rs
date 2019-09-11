use r68k_emu::{
    ram::AddressBus,
    ram::SUPERVISOR_PROGRAM,
    cpu::Exception
};

use r68k_tools::{
    PC,
    disassembler::disassemble,
    memory::Memory,
};

use super::CPU;

struct PhyMemory<'phy> {
    phy: &'phy CPU
}

impl<'phy> PhyMemory<'phy> {
    pub fn new(phy: &'phy CPU) -> PhyMemory<'phy> {
        PhyMemory { phy: phy }
    }
}

impl<'phy> Memory for PhyMemory<'phy> {
    fn offset(&self) -> u32 {
        self.phy.core.pc
    }
    fn data(&self) -> &[u8] {
        unimplemented!();
    }
    fn read_word(&self, pc: PC) -> u16 {
        self.phy.core.mem.read_word(SUPERVISOR_PROGRAM, pc.0) as u16
    }
    fn read_byte(&self, pc: PC) -> u8 {
        self.phy.core.mem.read_byte(SUPERVISOR_PROGRAM, pc.0) as u8
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

pub fn print_core_header(_tbx : &CPU) {
    println!(
        "        PC...... | IR.. | D0...... D1...... D2...... D3...... D4...... D5...... D6...... D7...... | A0...... A1...... A2...... A3...... A4...... A5...... A6...... A7...... | Next Instruction");
}

pub fn print_core_line(tbx : &CPU) {
    let c = &tbx.core;

    print!(
        "        {:08x} | {:04x} | {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} | {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} | ",
        c.pc, c.ir,
        c.dar[0], c.dar[1], c.dar[2], c.dar[3], c.dar[4], c.dar[5], c.dar[6], c.dar[7],
        c.dar[8], c.dar[9], c.dar[10], c.dar[11], c.dar[12], c.dar[13], c.dar[14], c.dar[15],
    );

    let mem = PhyMemory::new(&tbx);
    print!("${:04x}  ", mem.read_word(PC(c.pc)));
    if let Ok((_, inst)) = disassemble(PC(c.pc), &mem) {
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

pub fn print_core(tbx : &CPU) {
    println!("======================================");
    println!("PC:            {:08x}", tbx.core.pc);
    println!("SSP':          {:08x}", tbx.core.inactive_ssp);
    println!("USP':          {:08x}", tbx.core.inactive_usp);
    println!("IR:            {:08x}", tbx.core.ir);
    println!("D registers:   {:08x} {:08x} {:08x} {:08x}   {:08x} {:08x} {:08x} {:08x}",
        tbx.core.dar[0],
        tbx.core.dar[1],
        tbx.core.dar[2],
        tbx.core.dar[3],
        tbx.core.dar[4],
        tbx.core.dar[5],
        tbx.core.dar[6],
        tbx.core.dar[7]
    );
    println!("A registers:   {:08x} {:08x} {:08x} {:08x}   {:08x} {:08x} {:08x} {:08x}",
        tbx.core.dar[8+0],
        tbx.core.dar[8+1],
        tbx.core.dar[8+2],
        tbx.core.dar[8+3],
        tbx.core.dar[8+4],
        tbx.core.dar[8+5],
        tbx.core.dar[8+6],
        tbx.core.dar[8+7]
    );
    println!("S flag:        {:08x}", tbx.core.s_flag);
    println!("IRQ level:     {:08x}", tbx.core.irq_level);
    println!("INT mask:      {:08x}", tbx.core.int_mask);
    println!("X flag:        {:08x}", tbx.core.x_flag);
    println!("C flag:        {:08x}", tbx.core.c_flag);
    println!("V flag:        {:08x}", tbx.core.v_flag);
    println!("N flag:        {:08x}", tbx.core.n_flag);
    println!("Prefetch addr: {:08x}", tbx.core.prefetch_addr);
    println!("Prefetch data: {:08x}", tbx.core.prefetch_data);
    println!("not Z flag:    {:08x}", tbx.core.not_z_flag);
    println!("state:         {:?}", tbx.core.processing_state);
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