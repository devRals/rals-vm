use rals_vm_isa::{arch::Architecture, registers::Register, value::ImmediateValue};

use crate::cpu::{pc::ProgramCounter, sp::StackPointer};

/// RegisterFile is the place where all the CPU registers take place
pub struct RegisterFile<A: Architecture> {
    /// We have 16 register and everyone of the are the same size.
    pub general: [A::Word; 16],
    pub pc: ProgramCounter<A>,
    /// Stack pointer. A special register for showing where stack is located in the memory for the given program
    pub sp: StackPointer<A>,
    /// Frame pointer
    pub fp: A::Word,
}

impl<A: Architecture> RegisterFile<A> {
    /// Zero registers in CPU's cant be written and their values are always zero
    const ZERO_REGISTER: Register = Register::R0;

    pub fn new() -> Self {
        let sp = StackPointer::new();
        RegisterFile {
            pc: ProgramCounter::new(),
            general: [A::Word::ZERO; 16],
            fp: sp.get(),
            sp,
        }
    }

    pub fn reset(&mut self) {
        self.pc.reset();
        self.general.fill(A::Word::ZERO);
    }

    /// LDI (Load Immediate) changes the `dst` register value to given value
    pub const fn ldi(&mut self, dst: Register, src: A::Word) {
        self.set_reg(dst, src);
    }

    /// MOV (move) changes `dst` register value to `src` register value
    pub const fn mov(&mut self, dst: Register, src: Register) {
        self.set_reg(dst, self.get_reg(src));
    }

    /// INC (Increase) increases the given register value by one
    pub fn inc(&mut self, dst: Register) {
        let val = self.get_reg(dst);
        self.set_reg(dst, val.wrapping_add(A::Word::ONE));
    }

    /// DEC (Decrease) decreases the given register value by one
    pub fn dec(&mut self, dst: Register) {
        let val = self.get_reg(dst);
        self.set_reg(dst, val.wrapping_sub(A::Word::ONE));
    }

    pub const fn set_reg(&mut self, reg: Register, value: A::Word) {
        match reg {
            // zero registers physically cannot be changed
            Self::ZERO_REGISTER => {}
            Register::RSP => self.sp.set(value),
            Register::RFP => self.fp = value,

            Register::UnknownRegister => {
                panic!("rals-vm: recieved an unknown register. please update your code")
            }

            reg => self.general[reg as usize] = value,
        }
    }

    pub const fn get_reg(&self, reg: Register) -> A::Word {
        match reg {
            Self::ZERO_REGISTER => A::Word::ZERO,
            Register::RSP => self.sp.get(),
            Register::RFP => self.fp,

            Register::UnknownRegister => {
                panic!("rals-vm: recieved an unknown register. please update your code")
            }
            reg => self.general[reg as usize],
        }
    }
}
