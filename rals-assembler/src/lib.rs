use lalrpop_util::lalrpop_mod;

pub mod assembler;
pub mod ast;
// pub mod _errors;
// pub mod _parser;
// pub mod _symbol_table;
pub mod lexer;
// pub mod tokens;

mod instruction_resolvers;

lalrpop_mod!(pub grammar);
pub use crate::assembler::Assembler;
use crate::{
    assembler::AsmParseError, ast::AstProgram, grammar::ProgramParser, lexer::LexerAdapter,
};

pub fn parse(input: &str) -> Result<AstProgram, AsmParseError> {
    let lexer = LexerAdapter::new(input);
    ProgramParser::new().parse(lexer)
}
