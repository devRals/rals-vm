use lalrpop_util::lalrpop_mod;

pub mod assembler;
pub mod ast;
// pub mod _errors;
// pub mod _parser;
// pub mod _symbol_table;
pub mod lexer;
// pub mod tokens;

mod instruction_parsers;

lalrpop_mod!(pub grammar);
pub use crate::assembler::Assembler;
use crate::{
    assembler::AsmParseError, ast::AstProgram, grammar::ProgramParser, lexer::LexerAdapter,
};

pub fn parse(input: impl AsRef<str>) -> Result<AstProgram, AsmParseError> {
    let source = input.as_ref();
    let lexer = LexerAdapter::new(source);
    ProgramParser::new().parse(lexer)
}
