use crate::{
    ast::AstProgram,
    parser::{Parser, ParserResult},
    symbol_table::SymbolTable,
};

pub mod ast;
pub mod errors;
pub mod parser;
pub mod symbol_table;
pub mod tokenizer;
pub mod tokens;

pub struct Assembler<'src> {
    symbol_table: SymbolTable,
    source_str: &'src str,
}

impl<'src> Assembler<'src> {
    pub fn new(source_str: &'src str) -> Self {
        Assembler {
            symbol_table: SymbolTable::new(),
            source_str,
        }
    }

    pub fn parse(&mut self) -> ParserResult<AstProgram> {
        Parser::new(self.source_str, &mut self.symbol_table).parse_program()
    }
}
