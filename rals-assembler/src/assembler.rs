use std::{collections::HashMap, error::Error, fmt::Display, marker::PhantomData};

use rals_vm_isa::Encode;
use rals_vm_isa::arch::Architecture;
use rals_vm_isa::value::ImmediateValue;

use crate::ast::{AstInstruction, AstItem, AstProgram, Directive, HeaderSection, Instruction};
use crate::lexer::{LexError, Token};
use lalrpop_util::ParseError;

pub type AsmParseError = ParseError<usize, Token, LexError>;

pub struct ResolvedProgram<A: Architecture> {
    instructions: Vec<Instruction<A>>,
}

type Label = String;
pub(crate) type SymbolTable<A> = HashMap<Label, <A as Architecture>::Word>;

pub struct Assembler<A: Architecture> {
    symbol_table: SymbolTable<A>,
    _arch: PhantomData<A>,
}

impl<A: Architecture> Assembler<A> {
    pub fn new() -> Self {
        Assembler {
            symbol_table: SymbolTable::<A>::new(),
            _arch: PhantomData,
        }
    }

    fn resolve_data_section(&mut self) {}

    fn write_header(_out: &mut Vec<u8>) {}

    pub fn resolve_header(program: &AstProgram) -> Option<Result<HeaderSection, Pass1Error>> {
        let header_section = program
            .sections
            .iter()
            .find(|s| s.name.to_lowercase() == "header".to_string())?;

        let mut resolved = HeaderSection {
            directives: Vec::new(),
        };

        let all_directives = header_section
            .items
            .iter()
            .filter(|i| matches!(i, AstItem::Directive { .. }))
            .map(|d| match d {
                AstItem::Directive { key, value } => Directive {
                    key: key.clone(),
                    value: *value,
                },
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        resolved.directives = all_directives;

        Some(Ok(resolved))
    }

    fn resolve_instruction(&self, ins: AstInstruction) -> Result<Instruction<A>, Pass1Error> {
        match ins.mnemonic.to_lowercase().as_str() {
            "nop" => self.nop(ins),

            "inc" => self.inc(ins),
            "dec" => self.dec(ins),

            "add" => self.add(ins),
            "sub" => self.sub(ins),

            "addi" => self.addi(ins),
            "subi" => self.subi(ins),

            "and" => self.and(ins),
            "or" => self.or(ins),
            "xor" => self.xor(ins),

            "andi" => self.andi(ins),
            "ori" => self.ori(ins),
            "xori" => self.xori(ins),

            "shl" => self.shl(ins),
            "shr" => self.shr(ins),
            "sar" => self.sar(ins),

            "shli" => self.shli(ins),
            "shri" => self.shri(ins),
            "sari" => self.sari(ins),

            "cmp" => self.cmp(ins),

            "ldi" => self.ldi(ins),
            "mov" => self.mov(ins),

            "load" => self.load(ins),
            "store" => self.str(ins),

            "jmp" => self.jmp(ins),
            "jmr" => self.jmr(ins),

            "jo" => self.jo(ins),
            "jz" => self.jz(ins),
            "jc" => self.jc(ins),
            "js" => self.js(ins),

            "jno" => self.jno(ins),
            "jnz" => self.jnz(ins),
            "jnc" => self.jnc(ins),
            "jns" => self.jns(ins),

            "jro" => self.jro(ins),
            "jrz" => self.jrz(ins),
            "jrc" => self.jrc(ins),
            "jrs" => self.jrs(ins),

            "jrno" => self.jrno(ins),
            "jrnz" => self.jrnz(ins),
            "jrnc" => self.jrnc(ins),
            "jrns" => self.jrns(ins),

            "push" => self.push(ins),
            "pop" => self.pop(ins),

            "hlt" => self.hlt(ins),

            o => return Err(Pass1Error::UnknownOpCode(o.to_string())),
        }
    }

    pub fn pass1(&mut self, program: AstProgram) -> Result<ResolvedProgram<A>, Pass1Error> {
        let mut pc = A::Word::ZERO;
        let mut instructions = Vec::new();

        self.resolve_data_section();

        let text_section = program
            .sections
            .into_iter()
            .find(|s| s.name.to_lowercase() == "text".to_string())
            .ok_or(Pass1Error::TextSectionNotFound)?;

        for item in text_section.items {
            match item {
                AstItem::Directive { .. } => {}
                AstItem::Label(label_name) => {
                    self.symbol_table.insert(label_name, pc);
                }
                AstItem::Instruction(ins) => {
                    let resolved_ins = self.resolve_instruction(ins)?;
                    instructions.push(resolved_ins);
                    let (next, overflowed) = pc.overflowing_add(A::Word::ONE);
                    if overflowed {
                        return Err(Pass1Error::PCOverflowed);
                    } else {
                        pc = next;
                    }
                }
            }
        }

        let mut resolved = vec![];
        for i in instructions {
            resolved.push(i.resolve(&self.symbol_table)?)
        }

        Ok(ResolvedProgram {
            instructions: resolved,
        })
    }

    pub fn pass2(&mut self, program: ResolvedProgram<A>) -> Vec<u8> {
        let mut out = Vec::new();

        Assembler::<A>::write_header(&mut out);

        for ins in program.instructions {
            let mut ins_buf = vec![0u8; A::INSTRUCTION_SIZE];
            ins.encode(&mut ins_buf);
            out.extend(ins_buf);
        }

        out
    }

    pub fn assemble(&mut self, program: AstProgram) -> Result<Vec<u8>, AssembleError> {
        let resolved_program = self.pass1(program).map_err(AssembleError::Pass1Error)?;
        Ok(self.pass2(resolved_program))
    }
}

#[derive(Debug)]
pub enum AssembleError {
    Pass1Error(Pass1Error),
}

impl core::error::Error for AssembleError {}
impl core::fmt::Display for AssembleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pass1Error(e) => e.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum Pass1Error {
    Undefined(String),
    TextSectionNotFound,
    PCOverflowed,
    UnknownOpCode(String),
    UnknwonArchitecture(i64),
    WrongNumberOfOperands {
        opcode: String,
        expected: usize,
        got: usize,
    },
    WrongUsageOf {
        opcode: String,
        correct: String,
    },
    ImmediateOutOfRange {
        value: i64,
        max: usize,
    },
}

impl Error for Pass1Error {}
impl Display for Pass1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Pass1Error::*;
        match self {
            ImmediateOutOfRange { value, max } => {
                write!(f, "{value} is out of range of word value. Max is {max}")
            }
            TextSectionNotFound => write!(f, "`section .text` not found in the source"),
            PCOverflowed => write!(
                f,
                "program counter overflowed. if it continues advance later instructions positions will be wrong. Try using a bigger architecture word",
            ),
            UnknownOpCode(opcode) => write!(f, "There's no opcode such called `{opcode}`"),
            UnknwonArchitecture(arch) => write!(f, "There's no architecture such called `x{arch}`"),
            WrongUsageOf { opcode, correct } => {
                write!(f, "wrong usage of `{opcode}`. Correct usage: `{correct}`")
            }
            WrongNumberOfOperands {
                opcode,
                expected,
                got,
            } => write!(
                f,
                "Operation Code `{opcode}` expected `{expected}` amount of operands, got `{got}`"
            ),
            Undefined(ident) => write!(f, "`{ident}` is not defined"),
        }
    }
}
