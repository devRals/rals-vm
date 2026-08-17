use crate::{Decode, arch::Architecture, registers::Register};

pub struct Immediate<A: Architecture> {
    pub value: A::Word,
}

impl<A: Architecture> Decode for Immediate<A> {
    fn decode(ins: &[u8]) -> Self {
        Immediate {
            value: A::Word::decode(ins),
        }
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

    OR {
        dst: Register,
        lhs: Register,
        rhs: Register,
    } = 0x03,
    XOR {
        dst: Register,
        lhs: Register,
        rhs: Register,
    } = 0x04,
    AND {
        dst: Register,
        lhs: Register,
        rhs: Register,
    } = 0x05,

    SHL {
        dst: Register,
        lhs: Register,
        rhs: Register,
    } = 0x06,
    SHR {
        dst: Register,
        lhs: Register,
        rhs: Register,
    } = 0x07,
    SAR {
        dst: Register,
        lhs: Register,
        rhs: Register,
    } = 0x08,

    LDI {
        dst: Register,
        src: Immediate<A>,
    } = 0x70,
    MOV {
        dst: Register,
        src: Register,
    } = 0x71,

    /// Since a real computer can't understand what a Result is we can't directly use it.
    /// Instead we can define an instruction that should not be used in production.
    UnknownInstruction = 0xFE,
    HLT = 0xFF,
}

impl<A: Architecture> Instruction<A> {
    pub fn opcode(&self) -> u8 {
        use Instruction as I;

        match self {
            I::NOP => 0x00,

            I::ADD { .. } => 0x01,
            I::SUB { .. } => 0x02,

            I::OR { .. } => 0x03,
            I::XOR { .. } => 0x04,
            I::AND { .. } => 0x05,

            I::SHL { .. } => 0x06,
            I::SHR { .. } => 0x07,
            I::SAR { .. } => 0x08,

            I::LDI { .. } => 0x70,
            I::MOV { .. } => 0x71,

            I::UnknownInstruction => 0xFE,
            I::HLT => 0xFF,
        }
    }

    fn decode_from_opocode(opcode: u8, dst: Register, lhs: Register, rhs: Register) -> Self {
        use Instruction as I;
        match opcode {
            0x1 => I::ADD { dst, lhs, rhs },
            0x2 => I::SUB { dst, lhs, rhs },
            0x3 => I::OR { dst, lhs, rhs },
            0x4 => I::XOR { dst, lhs, rhs },
            0x5 => I::AND { dst, lhs, rhs },
            0x6 => I::SHL { dst, lhs, rhs },
            0x7 => I::SHR { dst, lhs, rhs },
            0x8 => I::SAR { dst, lhs, rhs },
            _ => I::UnknownInstruction,
        }
    }
}

impl<A: Architecture> Decode for Instruction<A> {
    fn decode(ins: &[u8]) -> Self
    where
        Self: Sized,
    {
        if ins.len() != A::INSTRUCTION_SIZE {
            panic!(
                "Failed decoding Instruction. Got Invalid length: {}. Update your code",
                ins.len()
            )
        }

        let opcode = ins[0];
        let operand1 = &ins[1..2];
        let operand2 = &ins[2..3];
        let operand3 = &ins[3..4];

        match opcode {
            0x0 => Instruction::NOP,
            0x1 | 0x2 | 0x3 | 0x4 | 0x5 | 0x6 | 0x7 => {
                let dst = Register::decode(operand1);
                let lhs = Register::decode(operand2);
                let rhs = Register::decode(operand3);

                Instruction::decode_from_opocode(opcode, dst, lhs, rhs)
            }
            0x70 => {
                let dst = Register::decode(operand1);
                let src = Immediate::decode(&ins[2..]);

                Instruction::LDI { dst, src }
            }
            0x71 => {
                let dst = Register::decode(operand1);
                let src = Register::decode(operand2);

                Instruction::MOV { dst, src }
            }
            0xFF => Instruction::HLT,
            _ => Instruction::UnknownInstruction,
        }
    }
}
