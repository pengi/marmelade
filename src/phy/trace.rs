use r68k_emu::{
    ram::AddressBus,
    ram::SUPERVISOR_PROGRAM
};

use r68k_tools::{
    PC,
    disassembler::disassemble,
    memory::Memory,
};

use super::{
    Phy,
    TrapHandler
};

struct PhyMemory<'phy, M : AddressBus, T : TrapHandler> {
    phy: &'phy Phy<M, T>
}

impl<'phy, M : AddressBus, T : TrapHandler> PhyMemory<'phy, M, T> {
    pub fn new(phy: &'phy Phy<M, T>) -> PhyMemory<'phy, M, T> {
        PhyMemory { phy: phy }
    }
}

impl<'phy, M : AddressBus, T : TrapHandler> Memory for PhyMemory<'phy, M, T> {
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

pub fn print_core_header<M : AddressBus, T : TrapHandler>(_tbx : &Phy<M, T>) {
    println!(
        "PC...... IR.. SSP'.... USP'....  D0...... D1...... D2...... D3...... D4...... D5...... D6...... D7......  A0...... A1...... A2...... A3...... A4...... A5...... A6...... A7......  Sflag... I# INT.mask Xflag... Cflag... Vflag... Nflag... Z'flag.. state");
}

pub fn print_core_line<M : AddressBus, T : TrapHandler>(tbx : &Phy<M, T>) {
    let c = &tbx.core;

    print!(
        "{:08x} {:04x} {:08x} {:08x}  {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x}  {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x}  {:08x} {:02x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:?}  ",
        c.pc, c.ir, c.inactive_ssp, c.inactive_usp,
        c.dar[0], c.dar[1], c.dar[2], c.dar[3], c.dar[4], c.dar[5], c.dar[6], c.dar[7],
        c.dar[8], c.dar[9], c.dar[10], c.dar[11], c.dar[12], c.dar[13], c.dar[14], c.dar[15],
        c.s_flag, c.irq_level, c.int_mask, c.x_flag, c.c_flag, c.v_flag, c.n_flag, c.not_z_flag, c.processing_state
    );

    let mem = PhyMemory::new(&tbx);
    print!("${:04x}  ", mem.read_word(PC(c.pc)));
    if let Ok((_, inst)) = disassemble(PC(c.pc), &mem) {
        let mut inst = format!("{}", inst);
        if let Some(delim) = inst.find("\t") {
            let arg = inst.split_off(delim);
            println!("{:10} {}", inst.trim(), arg.trim());
        } else {
            println!("{}", inst);
        }
    } else {
        println!("");
    }
}

pub fn print_core<M : AddressBus, T : TrapHandler>(tbx : &Phy<M, T>) {
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