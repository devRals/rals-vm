use core::marker::PhantomData;

use rals_vm_isa::{
    Decode, DecodeError, Encode,
    arch::Architecture,
    instructions::{Immediate, Operand},
    value::ImmediateValue,
};

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

impl Operand for Register {}

impl Encode for Register {
    fn encode(self, out: &mut [u8]) {
        out[0] = self as u8;
    }
}

impl Decode for Register {
    fn decode(ins: &[u8]) -> Result<Self, DecodeError> {
        let byte = *ins.first().ok_or(DecodeError::InvalidLength {
            expected: 1,
            got: 0,
        })?;

        Ok(match byte {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::R8,
            9 => Register::R9,
            10 => Register::R10,
            11 => Register::R11,
            12 => Register::R12,
            13 => Register::R13,
            14 => Register::R14,
            15 => Register::R15,
            unknown_reg_id => return Err(DecodeError::UnknownRegister { id: unknown_reg_id }),
        })
    }
}

#[repr(u8)]
pub enum Instruction<A: Architecture> {
    NOP = 0x00,

    ADD {
        dst: Register,
        lhs: Register,
        rhs: Register,
    } = 0x01,
    SUB {
        dst: Register,
        lhs: Register,
        rhs: Register,
    } = 0x02,

    LDI {
        dst: Register,
        src: Immediate<A>,
    } = 0x05,
    MOV {
        dst: Register,
        src: Register,
    } = 0x06,

    HLT = 0xFF,
}

impl<A: Architecture> Instruction<A> {
    pub fn opcode(&self) -> u8 {
        use Instruction as I;

        match self {
            I::NOP => 0x00,

            I::ADD { .. } => 0x01,
            I::SUB { .. } => 0x02,

            I::LDI { .. } => 0x05,
            I::MOV { .. } => 0x06,

            I::HLT => 0xFF,
        }
    }
}

impl<A: Architecture> Encode for Instruction<A> {
    fn encode(self, out: &mut [u8]) {
        use Instruction as I;

        out[0] = self.opcode();

        match self {
            I::NOP => {}

            I::ADD { dst, lhs, rhs } | I::SUB { dst, lhs, rhs } => {
                lhs.encode(&mut out[1..2]);
                rhs.encode(&mut out[2..3]);
                dst.encode(&mut out[3..4]);
            }

            I::LDI { dst, src } => {
                dst.encode(&mut out[1..2]);
                src.encode(&mut out[2..]);
            }

            I::MOV { dst, src } => {
                dst.encode(&mut out[1..2]);
                src.encode(&mut out[2..3]);
            }

            I::HLT => {}
        }
    }
}

impl<A: Architecture> Decode for Instruction<A> {
    fn decode(ins: &[u8]) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        if ins.len() != A::INSTRUCTION_SIZE {
            return Err(DecodeError::InvalidLength {
                expected: A::INSTRUCTION_SIZE,
                got: ins.len(),
            });
        }

        let opcode = ins[0];
        let operand1 = &ins[1..2];
        let operand2 = &ins[2..3];
        let operand3 = &ins[3..4];

        Ok(match opcode {
            0x0 => Instruction::NOP,
            0x1 => {
                let lhs = Register::decode(operand1)?;
                let rhs = Register::decode(operand2)?;
                let dst = Register::decode(operand3)?;

                Instruction::ADD { dst, lhs, rhs }
            }
            0x2 => {
                let lhs = Register::decode(operand1)?;
                let rhs = Register::decode(operand2)?;
                let dst = Register::decode(operand3)?;

                Instruction::SUB { dst, lhs, rhs }
            }
            0x5 => {
                let dst = Register::decode(operand1)?;
                let src = Immediate::decode(&ins[2..])?;

                Instruction::LDI { dst, src }
            }
            0x6 => {
                let dst = Register::decode(operand1)?;
                let src = Register::decode(operand2)?;

                Instruction::MOV { dst, src }
            }
            0xFF => Instruction::HLT,
            code => return Err(DecodeError::UnknownOpCode { code }),
        })
    }
}
