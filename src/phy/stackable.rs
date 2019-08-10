use r68k_emu::cpu::Core;

// Add for special core types too
use crate::types::OSType;

pub trait Stackable : Sized {
    fn stack_push(&self, core: &mut impl Core);
    fn stack_pop(core: &mut impl Core) -> Self;

    fn stack_replace(&self, core: &mut impl Core) -> Self {
        let res = Self::stack_pop(core);
        self.stack_push(core);
        res
    }
}

impl Stackable for u32 {
    fn stack_push(&self, core: &mut impl Core) {
        core.push_32(*self);
    }
    fn stack_pop(core: &mut impl Core) -> Self {
        core.pop_32()
    }
}

impl Stackable for u16 {
    fn stack_push(&self, core: &mut impl Core) {
        core.push_16(*self);
    }
    fn stack_pop(core: &mut impl Core) -> Self {
        core.pop_16()
    }
}

impl Stackable for u8 {
    // u8 is actually stored as u16, due to alignment
    fn stack_push(&self, core: &mut impl Core) {
        core.push_16(*self as u16);
    }
    fn stack_pop(core: &mut impl Core) -> Self {
        core.pop_16() as u8
    }
}

impl Stackable for i32 {
    fn stack_push(&self, core: &mut impl Core) {
        core.push_32(*self as u32);
    }
    fn stack_pop(core: &mut impl Core) -> Self {
        core.pop_32() as i32
    }
}

impl Stackable for i16 {
    fn stack_push(&self, core: &mut impl Core) {
        core.push_16(*self as u16);
    }
    fn stack_pop(core: &mut impl Core) -> Self {
        core.pop_16() as i16
    }
}

impl Stackable for i8 {
    // u8 is actually stored as u16, due to alignment
    fn stack_push(&self, core: &mut impl Core) {
        // Scale up first word length, then to unsigned to get sign extend correctly 
        core.push_16((*self as i16) as u16);
    }
    fn stack_pop(core: &mut impl Core) -> Self {
        // Just truncate when scaling down
        core.pop_16() as i8
    }
}

impl Stackable for OSType {
    fn stack_push(&self, core: &mut impl Core) {
        core.push_32(self.as_u32());
    }
    fn stack_pop(core: &mut impl Core) -> Self {
        OSType::from(core.pop_32())
    }
}

impl Stackable for () {
    fn stack_push(&self, _core: &mut impl Core) {
    }
    fn stack_pop(_core: &mut impl Core) -> Self {
    }
}