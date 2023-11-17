use crate::{binary_op_save_flags, Machine, OpSize};

macro_rules! div_save_flags {
    ($self:ident, $bit:ident, $save_bit:ident) => {{
        let (op2, op1) = unsafe { ($self.stack_pop::<$bit>() as $save_bit, $self.stack_pop::<$save_bit>()) };

        let qoutient = op1.wrapping_div(op2);
        let remainder = op1.wrapping_rem(op2);
        $self.set_rflags();

        unsafe { $self.stack_push(remainder as $bit); }
        unsafe { $self.stack_push(qoutient as $bit); }
    }}
}

use div_save_flags;

pub fn div(vm: &mut Machine, op_size: OpSize) {
    match op_size {
        OpSize::Qword => div_save_flags!(vm, u64, u128),
        OpSize::Dword => div_save_flags!(vm, u32, u64),
        OpSize::Word => div_save_flags!(vm, u16, u32),
        OpSize::Byte => div_save_flags!(vm, u16, u16),
    }
}

pub fn shr(vm: &mut Machine, op_size: OpSize) {
    match op_size {
        OpSize::Qword => binary_op_save_flags!(vm, u64, wrapping_div),
        OpSize::Dword => binary_op_save_flags!(vm, u32, wrapping_div),
        OpSize::Word => binary_op_save_flags!(vm, u16, wrapping_div),
        OpSize::Byte => binary_op_save_flags!(vm, u8, wrapping_div),
    }
}
