use std::{collections::HashMap, error::Error, fmt::Display, marker::PhantomData};

use rals_vm_isa::arch::{Arch32, Architecture};

use crate::ast::{AstInstruction, AstItem, AstProgram, Directive, HeaderSection, Instruction};
use crate::grammar::ProgramParser;
use crate::lexer::{LexError, LexerAdapter, Token};
use lalrpop_util::ParseError;

pub type AsmParseError = ParseError<usize, Token, LexError>;

pub struct ResolvedProgram {
    instructions: Vec<Instruction>,
}

type Label = String;
type Address = usize;
type SymbolTable = HashMap<Label, Address>;

pub struct Assembler<A: Architecture = Arch32> {
    symbol_table: SymbolTable,
    _arch: PhantomData<A>,
}

impl<A: Architecture> Assembler<A> {
    pub fn new() -> Self {
        Assembler {
            symbol_table: SymbolTable::new(),
            _arch: PhantomData,
        }
    }

    pub fn parse(input: &str) -> Result<AstProgram, AsmParseError> {
        let lexer = LexerAdapter::new(input);
        ProgramParser::new().parse(lexer)
    }

    fn resolve_data_section(&mut self) {}

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

    fn resolve_instruction(&self, ins: AstInstruction) -> Result<Instruction, Pass1Error> {
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
            "str" => self.str(ins),

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

            "hlt" => self.hlt(ins),

            o => return Err(Pass1Error::UnknownOpCode(o.to_string())),
        }
    }

    pub fn pass1(&mut self, program: AstProgram) -> Result<ResolvedProgram, Pass1Error> {
        let mut pc = 0;
        let mut resolved = ResolvedProgram {
            instructions: Vec::new(),
        };

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
                    resolved.instructions.push(resolved_ins);
                    pc += A::INSTRUCTION_SIZE;
                }
            }
        }

        Ok(resolved)
    }
}

#[derive(Debug)]
pub enum Pass1Error {
    TextSectionNotFound,
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
}

impl Error for Pass1Error {}
impl Display for Pass1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Pass1Error::*;
        match self {
            TextSectionNotFound => write!(f, "`section .text` not found in the source"),
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
        }
    }
}
