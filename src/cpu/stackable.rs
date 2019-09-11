use super::CPU;

pub trait Stackable : Sized {
    fn stack_push(&self, cpu: &mut CPU);
    fn stack_pop(cpu: &mut CPU) -> Self;

    fn stack_replace(&self, cpu: &mut CPU) -> Self {
        let res = Self::stack_pop(cpu);
        self.stack_push(cpu);
        res
    }
}

impl Stackable for u32 {
    fn stack_push(&self, cpu: &mut CPU) {
        cpu.core.push_32(*self);
    }
    fn stack_pop(cpu: &mut CPU) -> Self {
        cpu.core.pop_32()
    }
}

impl Stackable for u16 {
    fn stack_push(&self, cpu: &mut CPU) {
        cpu.core.push_16(*self);
    }
    fn stack_pop(cpu: &mut CPU) -> Self {
        cpu.core.pop_16()
    }
}

impl Stackable for u8 {
    // u8 is actually stored as u16, due to alignment
    fn stack_push(&self, cpu: &mut CPU) {
        cpu.core.push_16(*self as u16);
    }
    fn stack_pop(cpu: &mut CPU) -> Self {
        cpu.core.pop_16() as u8
    }
}

impl Stackable for i32 {
    fn stack_push(&self, cpu: &mut CPU) {
        cpu.core.push_32(*self as u32);
    }
    fn stack_pop(cpu: &mut CPU) -> Self {
        cpu.core.pop_32() as i32
    }
}

impl Stackable for i16 {
    fn stack_push(&self, cpu: &mut CPU) {
        cpu.core.push_16(*self as u16);
    }
    fn stack_pop(cpu: &mut CPU) -> Self {
        cpu.core.pop_16() as i16
    }
}

impl Stackable for i8 {
    // u8 is actually stored as u16, due to alignment
    fn stack_push(&self, cpu: &mut CPU) {
        // Scale up first word length, then to unsigned to get sign extend correctly 
        cpu.core.push_16((*self as i16) as u16);
    }
    fn stack_pop(cpu: &mut CPU) -> Self {
        // Just truncate when scaling down
        cpu.core.pop_16() as i8
    }
}

impl Stackable for () {
    fn stack_push(&self, _cpu: &mut CPU) {
    }
    fn stack_pop(_cpu: &mut CPU) -> Self {
    }
}