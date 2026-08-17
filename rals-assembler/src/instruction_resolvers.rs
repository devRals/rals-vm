use std::collections::HashMap;

use rals_vm_isa::{Encode, arch::Architecture, value::ImmediateValue};

use crate::{
    Assembler,
    assembler::{Pass1Error, Pass2Error},
    ast::{AstInstruction, AstJumpTarget, AstOperand::*, Instruction},
};

impl Instruction {
    pub(super) fn resolve(self, symbol_table: &HashMap<String, usize>) -> Result<Self, Pass2Error> {
        use AstJumpTarget as T;
        use Instruction as I;

        macro_rules! resolve_ins {
            ($($unresolved_ins: ident => $resolved_ins: ident ),* $(,)?) => {
                match self {
                $(
                    I::$unresolved_ins { target } => match target {
                        T::Addr(addr) => Ok(I::$resolved_ins { addr }),
                        T::Label(label) => Ok(I::$resolved_ins {
                            addr: *symbol_table
                                .get(&label)
                                .ok_or(Pass2Error::Undefined(label))?,
                        }),
                    },

                )*
                    i => Ok(i),
                }
            };
        }

        resolve_ins! [
            JMPUnresolved => JMP,

            JSUnresolved => JS,
            JOUnresolved => JO,
            JCUnresolved => JC,
            JZUnresolved => JZ,

            JNSUnresolved => JNS,
            JNOUnresolved => JNO,
            JNCUnresolved => JNC,
            JNZUnresolved => JNZ,
        ]
    }

    fn opcode(&self) -> u8 {
        use Instruction::*;

        match self {
            NOP => 0x00,

            ADD { .. } => 0x01,
            SUB { .. } => 0x02,

            OR { .. } => 0x03,
            XOR { .. } => 0x04,
            AND { .. } => 0x05,

            SHL { .. } => 0x06,
            SHR { .. } => 0x07,
            SAR { .. } => 0x08,

            ADDI { .. } => 0x09,
            SUBI { .. } => 0x0a,

            ORI { .. } => 0x0b,
            XORI { .. } => 0x0c,
            ANDI { .. } => 0x0d,

            SHLI { .. } => 0x0e,
            SHRI { .. } => 0x0f,
            SARI { .. } => 0x10,

            CMP { .. } => 0x20,

            LDI { .. } => 0x70,
            MOV { .. } => 0x71,

            INC { .. } => 0x72,
            DEC { .. } => 0x73,

            JO { .. } => 0x74,
            JC { .. } => 0x75,
            JS { .. } => 0x76,
            JZ { .. } => 0x77,

            JNO { .. } => 0x78,
            JNC { .. } => 0x79,
            JNS { .. } => 0x7a,
            JNZ { .. } => 0x7b,

            JRO { .. } => 0x7c,
            JRC { .. } => 0x7d,
            JRS { .. } => 0x7e,
            JRZ { .. } => 0x7f,

            JRNO { .. } => 0x80,
            JRNC { .. } => 0x81,
            JRNS { .. } => 0x82,
            JRNZ { .. } => 0x83,

            JMP { .. } => 0x84,
            JMR { .. } => 0x85,

            LOAD { .. } => 0xD0,
            STR { .. } => 0xD1,

            PUSH { .. } => 0xE0,
            POP { .. } => 0xE1,

            JOUnresolved { .. }
            | JCUnresolved { .. }
            | JSUnresolved { .. }
            | JZUnresolved { .. }
            | JNOUnresolved { .. }
            | JNCUnresolved { .. }
            | JNSUnresolved { .. }
            | JNZUnresolved { .. }
            | JMPUnresolved { .. } => 0xFE, // Unknown instruction code

            HLT => 0xFF,
        }
    }
}

impl<A: Architecture> Encode<A> for Instruction {
    fn encode(self, out: &mut [u8]) {
        out.fill(0);
        let opcode = &mut out[0];
        *opcode = self.opcode();

        use Instruction::*;

        match self {
            NOP | HLT => {}
            ADD { dst, lhs, rhs }
            | SUB { dst, lhs, rhs }
            | OR { dst, lhs, rhs }
            | XOR { dst, lhs, rhs }
            | AND { dst, lhs, rhs }
            | SHL { dst, lhs, rhs }
            | SHR { dst, lhs, rhs }
            | SAR { dst, lhs, rhs } => {
                out[1] = dst;
                out[2] = lhs;
                out[3] = rhs;
            }

            ADDI { dst, lhs, imm }
            | SUBI { dst, lhs, imm }
            | ORI { dst, lhs, imm }
            | XORI { dst, lhs, imm }
            | ANDI { dst, lhs, imm }
            | SHLI { dst, lhs, imm }
            | SHRI { dst, lhs, imm }
            | SARI { dst, lhs, imm } => {
                out[1] = dst;
                out[2] = lhs;
                let value = <A::Word as ImmediateValue>::try_from_i64(imm)
                    .expect("Failed to encode an immediate value. Fix you code");
                out[3..A::Word::BYTES].copy_from_slice(value.to_bytes().as_ref());
            }
            _ => {}
        }
    }
}
macro_rules! make_resolver {
    ($name: ident: {
        operand_count: $op_count: expr,
        usage: $usage: expr,
        $(
            $ops: pat => $expr: expr $(,)?
        )*
    }) => {
        pub(crate) fn $name(&self, ins: AstInstruction) -> Result<Instruction, Pass1Error> {
            if ins.operands.len() != $op_count {
                return Err(Pass1Error::WrongNumberOfOperands {
                    opcode: stringify!($name).to_string(),
                    expected: $op_count,
                    got: ins.operands.len(),
                });
            }
            let operands = (
                ins.operands.get(0).cloned(),
                ins.operands.get(1).cloned(),
                ins.operands.get(2).cloned(),
            );

            match operands {
                $(
                    $ops => Ok($expr),
                )*
                _ => Err(Pass1Error::WrongUsageOf {
                    opcode: stringify!($name).to_string(),
                    correct: format!("{} {}", stringify!($name), $usage)
                }),
            }
        }
    };
}

impl<A: Architecture> Assembler<A> {
    make_resolver!(nop: {
        operand_count: 0,
        usage: "",
        (None, None, None) => Instruction::NOP
    });

    make_resolver!(inc : {
        operand_count: 1,
        usage: "<reg>",
        (Some(Reg(reg)), None, None) => Instruction::INC {reg},
    });
    make_resolver!(dec : {
        operand_count: 1,
        usage: "<reg>",
        (Some(Reg(reg)), None, None) => Instruction::DEC {reg},
    });

    make_resolver!(ldi : {
        operand_count: 2,
        usage: "<reg>, <imm>",
        (Some(Reg(reg)), Some(Imm(imm)), None) => Instruction::LDI { reg, imm }
    });
    make_resolver!(mov : {
        operand_count: 2,
        usage: "<reg>, <reg>",
        (Some(Reg(r1)), Some(Reg(r2)), None) => Instruction::MOV { r1, r2 }
    });

    make_resolver!(add : {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::ADD {dst, lhs, rhs},
    });
    make_resolver!(sub : {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::SUB {dst, lhs, rhs},
    });

    make_resolver!(addi : {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::ADDI {dst, lhs, imm},
    });
    make_resolver!(subi : {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::SUBI {dst, lhs, imm},
    });

    make_resolver!(shl: {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::SHL {dst, lhs, rhs},
    });
    make_resolver!(shr: {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::SHR {dst, lhs, rhs},
    });
    make_resolver!(sar: {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::SAR {dst, lhs, rhs},
    });

    make_resolver!(shli: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::SHLI {dst, lhs, imm},
    });
    make_resolver!(shri: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::SHRI {dst, lhs, imm},
    });
    make_resolver!(sari: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::SARI {dst, lhs, imm},
    });

    make_resolver!(or: {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::OR {dst, lhs, rhs},
    });
    make_resolver!(xor: {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::XOR {dst, lhs, rhs},
    });
    make_resolver!(and: {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::AND {dst, lhs, rhs},
    });

    make_resolver!(ori: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::ORI {dst, lhs, imm},
    });
    make_resolver!(xori: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::XORI {dst, lhs, imm},
    });
    make_resolver!(andi: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(dst)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::ANDI {dst, lhs, imm},
    });

    make_resolver!(cmp: {
        operand_count: 2,
        usage: "<reg>, <reg>",
        (Some(Reg(r1)), Some(Reg(r2)), None) => Instruction::CMP { r1, r2 }
    });

    make_resolver!(load: {
        operand_count: 2,
        usage: "<reg>, [<reg> +/- <imm>]",
        (Some(Reg(dst)), Some(Deref { base, offset }), None) => Instruction::LOAD { dst, target_base: base, target_offset: offset }
    });

    make_resolver!(str: {
        operand_count: 2,
        usage: "<reg>, [<reg> +/- <imm>]",
        (Some(Reg(dst)), Some(Deref { base, offset }), None) => Instruction::STR { dst, target_base: base, target_offset: offset }
    });

    make_resolver!(jmp : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JMPUnresolved { target: AstJumpTarget::Addr(imm as usize) },
        (Some(Label(label)), None, None) => Instruction::JMPUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(jmr: {
        operand_count: 1,
        usage: "<signed_imm>",
        (Some(Imm(amount)), None, None) => Instruction::JMR { amount },
    });

    make_resolver!(jo : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JOUnresolved { target: AstJumpTarget::Addr(imm as usize) },
        (Some(Label(label)), None, None) => Instruction::JOUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(jc : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JCUnresolved { target: AstJumpTarget::Addr(imm as usize) },
        (Some(Label(label)), None, None) => Instruction::JCUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(jz : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JZUnresolved { target: AstJumpTarget::Addr(imm as usize) },
        (Some(Label(label)), None, None) => Instruction::JZUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(js : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JSUnresolved { target: AstJumpTarget::Addr(imm as usize) },
        (Some(Label(label)), None, None) => Instruction::JSUnresolved { target: AstJumpTarget::Label(label) },
    });

    make_resolver!(jno : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JNOUnresolved { target: AstJumpTarget::Addr(imm as usize) },
        (Some(Label(label)), None, None) => Instruction::JNOUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(jnc : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JNCUnresolved { target: AstJumpTarget::Addr(imm as usize) },
        (Some(Label(label)), None, None) => Instruction::JNCUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(jnz : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JNZUnresolved { target: AstJumpTarget::Addr(imm as usize) },
        (Some(Label(label)), None, None) => Instruction::JNZUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(jns : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JNSUnresolved { target: AstJumpTarget::Addr(imm as usize) },
        (Some(Label(label)), None, None) => Instruction::JNSUnresolved { target: AstJumpTarget::Label(label) },
    });

    make_resolver!(jro : {
        operand_count: 1,
        usage: "<signed_imm>",
        (Some(Imm(amount)), None, None) => Instruction::JRO { amount },
    });
    make_resolver!(jrc : {
        operand_count: 1,
        usage: "<signed_imm>",
        (Some(Imm(amount)), None, None) => Instruction::JRC { amount },
    });
    make_resolver!(jrz : {
        operand_count: 1,
        usage: "<signed_imm>",
        (Some(Imm(amount)), None, None) => Instruction::JRZ { amount },
    });
    make_resolver!(jrs : {
        operand_count: 1,
        usage: "<signed_imm>",
        (Some(Imm(amount)), None, None) => Instruction::JRS { amount },
    });

    make_resolver!(jrno : {
        operand_count: 1,
        usage: "<signed_imm>",
        (Some(Imm(amount)), None, None) => Instruction::JRNO { amount },
    });
    make_resolver!(jrnc : {
        operand_count: 1,
        usage: "<signed_imm>",
        (Some(Imm(amount)), None, None) => Instruction::JRNC { amount },
    });
    make_resolver!(jrnz : {
        operand_count: 1,
        usage: "<signed_imm>",
        (Some(Imm(amount)), None, None) => Instruction::JRNZ { amount },
    });
    make_resolver!(jrns : {
        operand_count: 1,
        usage: "<signed_imm>",
        (Some(Imm(amount)), None, None) => Instruction::JRNS { amount },
    });

    make_resolver!(push: {
        operand_count: 1,
        usage: "<reg>",
        (Some(Reg(reg)), None, None) => Instruction::PUSH { reg },
    });
    make_resolver!(pop: {
        operand_count: 1,
        usage: "<reg>",
        (Some(Reg(reg)), None, None) => Instruction::POP { reg },
    });

    make_resolver!(hlt : {
        operand_count: 0,
        usage: "",
        (None, None, None) => Instruction::HLT,
    });
}
