use rals_vm_isa::arch::Architecture;

use crate::{
    Assembler,
    assembler::Pass1Error,
    ast::{AstInstruction, AstJumpTarget, AstOperand::*, Instruction},
};

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
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::SHL {reg, lhs, rhs},
    });
    make_resolver!(shr: {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::SHR {reg, lhs, rhs},
    });
    make_resolver!(sar: {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::SAR {reg, lhs, rhs},
    });

    make_resolver!(shli: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::SHLI {reg, lhs, imm},
    });
    make_resolver!(shri: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::SHRI {reg, lhs, imm},
    });
    make_resolver!(sari: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::SARI {reg, lhs, imm},
    });

    make_resolver!(or: {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::OR {reg, lhs, rhs},
    });
    make_resolver!(xor: {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::XOR {reg, lhs, rhs},
    });
    make_resolver!(and: {
        operand_count: 3,
        usage: "<reg>, <reg>, <reg>",
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Reg(rhs))) => Instruction::AND {reg, lhs, rhs},
    });

    make_resolver!(ori: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::ORI {reg, lhs, imm},
    });
    make_resolver!(xori: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::XORI {reg, lhs, imm},
    });
    make_resolver!(andi: {
        operand_count: 3,
        usage: "<reg>, <reg>, <imm>",
        (Some(Reg(reg)), Some(Reg(lhs)), Some(Imm(imm))) => Instruction::ANDI {reg, lhs, imm},
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
        (Some(Imm(imm)), None, None) => Instruction::JMPUnresolved { target: AstJumpTarget::Addr(imm as u64) },
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
        (Some(Imm(imm)), None, None) => Instruction::JOUnresolved { target: AstJumpTarget::Addr(imm as u64) },
        (Some(Label(label)), None, None) => Instruction::JOUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(jc : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JCUnresolved { target: AstJumpTarget::Addr(imm as u64) },
        (Some(Label(label)), None, None) => Instruction::JCUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(jz : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JZUnresolved { target: AstJumpTarget::Addr(imm as u64) },
        (Some(Label(label)), None, None) => Instruction::JZUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(js : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JSUnresolved { target: AstJumpTarget::Addr(imm as u64) },
        (Some(Label(label)), None, None) => Instruction::JSUnresolved { target: AstJumpTarget::Label(label) },
    });

    make_resolver!(jno : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JNOUnresolved { target: AstJumpTarget::Addr(imm as u64) },
        (Some(Label(label)), None, None) => Instruction::JNOUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(jnc : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JNCUnresolved { target: AstJumpTarget::Addr(imm as u64) },
        (Some(Label(label)), None, None) => Instruction::JNCUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(jnz : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JNZUnresolved { target: AstJumpTarget::Addr(imm as u64) },
        (Some(Label(label)), None, None) => Instruction::JNZUnresolved { target: AstJumpTarget::Label(label) },
    });
    make_resolver!(jns : {
        operand_count: 1,
        usage: "(<imm>|<label>)",
        (Some(Imm(imm)), None, None) => Instruction::JNSUnresolved { target: AstJumpTarget::Addr(imm as u64) },
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

    make_resolver!(hlt : {
        operand_count: 0,
        usage: "",
        (None, None, None) => Instruction::HLT,
    });
}
