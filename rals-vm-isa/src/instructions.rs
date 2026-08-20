use crate::{Decode, arch::Architecture, registers::Register};

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

    ADDI {
        dst: Register,
        lhs: Register,
        imm: A::Word,
    } = 0x09,
    SUBI {
        dst: Register,
        lhs: Register,
        imm: A::Word,
    } = 0x0a,

    ORI {
        dst: Register,
        lhs: Register,
        imm: A::Word,
    } = 0x0b,
    XORI {
        dst: Register,
        lhs: Register,
        imm: A::Word,
    } = 0x0c,
    ANDI {
        dst: Register,
        lhs: Register,
        imm: A::Word,
    } = 0x0d,

    SHLI {
        dst: Register,
        lhs: Register,
        imm: A::Word,
    } = 0x0e,
    SHRI {
        dst: Register,
        lhs: Register,
        imm: A::Word,
    } = 0x0f,
    SARI {
        dst: Register,
        lhs: Register,
        imm: A::Word,
    } = 0x10,

    CMP {
        r1: Register,
        r2: Register,
    } = 0x20,

    LDI {
        dst: Register,
        src: A::Word,
    } = 0x70,
    MOV {
        dst: Register,
        src: Register,
    } = 0x71,

    INC {
        reg: Register,
    } = 0x72,
    DEC {
        reg: Register,
    } = 0x73,

    JO {
        addr: A::Word,
    } = 0x74,
    JC {
        addr: A::Word,
    } = 0x75,
    JS {
        addr: A::Word,
    } = 0x76,
    JZ {
        addr: A::Word,
    } = 0x77,

    JNO {
        addr: A::Word,
    } = 0x78,
    JNC {
        addr: A::Word,
    } = 0x79,
    JNS {
        addr: A::Word,
    } = 0x7a,
    JNZ {
        addr: A::Word,
    } = 0x7b,

    JRO {
        amount: A::Word,
    } = 0x7c,
    JRC {
        amount: A::Word,
    } = 0x7d,
    JRS {
        amount: A::Word,
    } = 0x7e,
    JRZ {
        amount: A::Word,
    } = 0x7f,

    JRNO {
        amount: A::Word,
    } = 0x80,
    JRNC {
        amount: A::Word,
    } = 0x81,
    JRNS {
        amount: A::Word,
    } = 0x82,
    JRNZ {
        amount: A::Word,
    } = 0x83,

    JMP {
        addr: A::Word,
    } = 0x84,
    JMR {
        amount: A::Word,
    } = 0x85,

    LOAD {
        dst: Register,
        target_base: Register,
        target_index: Register,
        target_displacement: A::Word,
    } = 0xD0,
    STORE {
        target: Register,
        dst_base: Register,
        dst_index: Register,
        dst_displacement: A::Word,
    } = 0xD1,

    PUSH {
        reg: Register,
    } = 0xE0,
    POP {
        reg: Register,
    } = 0xE1,
    /// Since a real computer can't understand what a Result is we can't directly use it.
    /// Instead we can define an instruction that should not be used in production.
    UnknownInstruction = 0xFE,
    HLT = 0xFF,
}

impl<A: Architecture> Instruction<A> {
    fn match_from_opcode(opcode: u8, dst: Register, lhs: Register, rhs: Register) -> Self {
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

    fn match_from_opcode_immediate(opcode: u8, dst: Register, lhs: Register, imm: A::Word) -> Self {
        use Instruction as I;
        match opcode {
            0x9 => I::ADDI { dst, lhs, imm },
            0xa => I::SUBI { dst, lhs, imm },
            0xb => I::ORI { dst, lhs, imm },
            0xc => I::XORI { dst, lhs, imm },
            0xd => I::ANDI { dst, lhs, imm },
            0xe => I::SHLI { dst, lhs, imm },
            0xf => I::SHRI { dst, lhs, imm },
            0x10 => I::SARI { dst, lhs, imm },
            _ => I::UnknownInstruction,
        }
    }

    fn match_from_opcode_jmps(opcode: u8, addr: A::Word) -> Instruction<A> {
        use Instruction as I;
        match opcode {
            0x84 => I::JMP { addr },

            0x74 => I::JO { addr },
            0x75 => I::JC { addr },
            0x76 => I::JS { addr },
            0x77 => I::JZ { addr },

            0x78 => I::JNO { addr },
            0x79 => I::JNC { addr },
            0x7a => I::JNS { addr },
            0x7b => I::JNZ { addr },
            _ => I::UnknownInstruction,
        }
    }
    fn match_from_opcode_jmps_relative(opcode: u8, amount: A::Word) -> Instruction<A> {
        use Instruction as I;
        match opcode {
            0x85 => I::JMR { amount },

            0x7c => I::JRO { amount },
            0x7d => I::JRC { amount },
            0x7e => I::JRS { amount },
            0x7f => I::JRZ { amount },

            0x80 => I::JRNO { amount },
            0x81 => I::JRNC { amount },
            0x82 => I::JRNS { amount },
            0x83 => I::JRNZ { amount },
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
            // NOP
            0x0 => Instruction::NOP,

            // Arithmetical & Logical
            0x1..=0x8 => {
                let dst = Register::decode(operand1);
                let lhs = Register::decode(operand2);
                let rhs = Register::decode(operand3);

                Instruction::match_from_opcode(opcode, dst, lhs, rhs)
            }

            // Arithmetical & Logical (Immediate)
            0x9..=0x10 => {
                let dst = Register::decode(operand1);
                let lhs = Register::decode(operand2);
                let imm = A::Word::decode(&ins[3..]);

                Instruction::match_from_opcode_immediate(opcode, dst, lhs, imm)
            }

            // CMP
            0x20 => {
                let r1 = Register::decode(operand1);
                let r2 = Register::decode(operand2);

                Instruction::CMP { r1, r2 }
            }

            // LDI
            0x70 => {
                let dst = Register::decode(operand1);
                let src = A::Word::decode(&ins[2..]);

                Instruction::LDI { dst, src }
            }

            // MOV
            0x71 => {
                let dst = Register::decode(operand1);
                let src = Register::decode(operand2);

                Instruction::MOV { dst, src }
            }

            // INC / DEC
            0x72 | 0x73 => {
                let reg = Register::decode(operand1);

                match opcode {
                    0x72 => Instruction::INC { reg },
                    0x73 => Instruction::DEC { reg },
                    _ => unreachable!(),
                }
            }

            // Jumps (Direct) & JMP
            0x74..=0x7b | 0x84 => {
                let addr = A::Word::decode(&ins[1..]);
                Instruction::match_from_opcode_jmps(opcode, addr)
            }

            // Jumps (Relative) & JMR
            0x7c..=0x83 | 0x85 => {
                let amount = A::Word::decode(&ins[1..]);
                Instruction::match_from_opcode_jmps_relative(opcode, amount)
            }

            // LOAD
            0xd0 => {
                let dst = Register::decode(operand1);
                let (target_base, target_index) =
                    (Register::decode(operand2), Register::decode(operand3));
                let target_displacement = A::Word::decode(&ins[4..]);

                Instruction::LOAD {
                    dst,
                    target_base,
                    target_index,
                    target_displacement,
                }
            }

            // STORE
            0xd1 => {
                let (dst_base, dst_index) =
                    (Register::decode(operand2), Register::decode(operand3));
                let target = Register::decode(operand1);
                let dst_displacement = A::Word::decode(&ins[4..]);

                Instruction::STORE {
                    target,
                    dst_base,
                    dst_index,
                    dst_displacement,
                }
            }

            // PUSH & POP
            0xe0 | 0xe1 => {
                let reg = Register::decode(operand1);

                match opcode {
                    0xe0 => Instruction::PUSH { reg },
                    0xe1 => Instruction::POP { reg },
                    _ => unreachable!(),
                }
            }

            // HLT
            0xFF => Instruction::HLT,
            _ => Instruction::UnknownInstruction,
        }
    }
}
