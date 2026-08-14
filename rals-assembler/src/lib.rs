use crate::{
    lexer::LexerAdapter,
    // _symbol_table::SymbolTable,
};

use lalrpop_util::lalrpop_mod;

pub mod ast;
// pub mod _errors;
// pub mod _parser;
// pub mod _symbol_table;
pub mod lexer;
pub mod tokens;

lalrpop_mod!(pub grammar);

pub fn parse(input: &str) -> Result<ast::Program, String> {
    let lexer = LexerAdapter::new(input);
    grammar::ProgramParser::new()
        .parse(lexer)
        .map_err(|e| format!("{e:?}"))
}
