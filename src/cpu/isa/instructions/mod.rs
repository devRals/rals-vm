use crate::cpu::{
    Decode, DecodeError, Encode,
    isa::{arch::Architecture, value::ImmediateValue},
    reg_file::Register,
};

pub trait Operand: Encode + Decode {}

pub struct Immediate<A: Architecture> {
    pub value: A::Word,
}

impl<A: Architecture> Encode for Immediate<A> {
    fn encode(self, out: &mut [u8]) {
        out.copy_from_slice(self.value.to_bytes().as_ref());
    }
}

impl<A: Architecture> Decode for Immediate<A> {
    fn decode(ins: &[u8]) -> Result<Self, DecodeError> {
        Ok(Immediate {
            value: A::Word::decode(ins)?,
        })
    }
}

impl<A: Architecture> Operand for Immediate<A> {}

impl Operand for Register {}

impl Encode for Register {
    fn encode(self, out: &mut [u8]) {
        out[0] = self as u8;
    }
}

impl Decode for Register {
    fn decode(ins: &[u8]) -> Result<Self, DecodeError> {
        let byte = *ins.first().ok_or(DecodeError::InvalidLength)?;

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
            _ => return Err(DecodeError::UnknownRegister),
        })
    }
}

pub enum Instruction<A: Architecture> {
    NOP,

    ADD {
        dst: Register,
        lhs: Register,
        rhs: Register,
    },
    SUB {
        dst: Register,
        lhs: Register,
        rhs: Register,
    },

    LDI {
        dst: Register,
        src: Immediate<A>,
    },
    MOV {
        dst: Register,
        src: Register,
    },

    HLT,
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
                dst.encode(&mut out[3..]);
            }

            I::LDI { dst, src } => {
                dst.encode(&mut out[1..2]);
                src.encode(&mut out[2..3]);
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
            return Err(DecodeError::InvalidLength);
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
            _ => return Err(DecodeError::UnknownRegister),
        })
    }
}

pub struct RawInstruction<A: Architecture> {
    pub data: A::Instruction,
}

impl<A: Architecture> RawInstruction<A> {
    pub const fn new(ins: A::Instruction) -> Self {
        RawInstruction { data: ins }
    }
}
