use core::marker::PhantomData;

use super::{isa::arch::Architecture, isa::value::ImmediateValue};

/// PC (Program Counter not Personal Computer) is a physical&hardware CPU Component that
/// tells us which instruction were currently executing. Every time the
/// instruction loop (fetch -> decode -> execute -> repeat) ends
/// PC increases which means we skipped to the next instruction in a program
#[derive(Default)]
pub struct ProgramCounter<A: Architecture> {
    pub stopped: bool,
    counter: u64,
    _arch: PhantomData<A>,
}

impl<A: Architecture> ProgramCounter<A> {
    pub const fn new() -> Self {
        ProgramCounter {
            counter: 0,
            stopped: false,
            _arch: PhantomData,
        }
    }

    pub fn advance(&mut self) {
        if !self.stopped {
            self.counter += A::INSTRUCTION_SIZE as u64;
        }
    }

    /// JMP (Jump) goes to the given instruction address and continues from at that point
    pub fn jmp(&mut self, addr: u64) {
        if !self.stopped {
            self.counter = addr;
        }
    }

    /// JMR (Jump Relative) skips `amount` amount of instructions and continues from at that point
    pub fn jmr(&mut self, amount: u64) {
        if !self.stopped {
            self.counter += amount * A::INSTRUCTION_SIZE as u64;
        }
    }

    pub fn hlt(&mut self) {
        self.stopped = true;
    }

    pub const fn get(&self) -> u64 {
        self.counter
    }

    pub const fn peek(&self) -> u64 {
        self.counter + A::INSTRUCTION_SIZE as u64
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

/// RegisterFile is the place where all the CPU registers take place
pub struct RegisterFile<A: Architecture> {
    /// We have 16 register and everyone of the are the same size.
    pub general: [A::Word; 16],
    pub pc: ProgramCounter<A>,
}

impl<A: Architecture> RegisterFile<A> {
    pub const fn new() -> Self {
        RegisterFile {
            pc: ProgramCounter::new(),
            general: [A::Word::ZERO; 16],
        }
    }

    pub fn reset(&mut self) {
        self.pc.counter = 0;
        self.general.fill(A::Word::ZERO);
    }

    /// LDI (Load Immediate) changes the `dst` register value to given value
    pub fn ldi(&mut self, dst: Register, src: A::Word) {
        self.general[dst as usize] = src;
    }

    /// MOV (move) changes `dst` register value to `src` register value
    pub fn mov(&mut self, dst: Register, src: Register) {
        self.general[dst as usize] = self.general[src as usize]
    }

    /// INC (Increase) increases the given register value
    pub fn inc(&mut self, dst: Register) {
        self.general[dst as usize] = self.general[dst as usize] + A::Word::ONE
    }
}
