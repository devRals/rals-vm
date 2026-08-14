use std::io;

use crate::tokens::{SourcePos, Span, TokenType};

pub struct ErrorWriter<'src> {
    source: &'src str,
}

impl<'src> ErrorWriter<'src> {
    pub const fn new(source: &'src str) -> Self {
        ErrorWriter { source }
    }

    pub fn write_parse_error<W: io::Write>(&self, out: &mut W, err: &ParseError) -> io::Result<()> {
        write!(out, "error[{}:{}] ", err.pos.line, err.pos.column)?;

        match err.kind {
            ParserErrorKind::SyntaxError { expected, got } => write!(
                out,
                "Expected token type `{:?}` but got `{:?}`",
                expected, got
            )?,

            ParserErrorKind::NotAValidSection { span } => {
                let name = &self.source[span.start..span.start + span.len];

                write!(
                    out,
                    "\"{}\" is not a valid section name. Valid section names: \"header\", \"text\"",
                    name
                )?
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct ParseError {
    pos: SourcePos,
    kind: ParserErrorKind,
}

impl ParseError {
    pub const fn new(kind: ParserErrorKind, pos: SourcePos) -> Self {
        Self { pos, kind }
    }
}

#[derive(Clone, Copy)]
pub enum ParserErrorKind {
    SyntaxError { expected: TokenType, got: TokenType },
    NotAValidSection { span: Span },
}
