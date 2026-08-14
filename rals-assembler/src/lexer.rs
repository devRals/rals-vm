use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\f]+")]
#[logos(skip r";[^\n]*")]
pub enum Token {
    #[regex("\n+")]
    Newline,

    #[token("section")]
    Section,

    #[regex(r"\.[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice()[1..].to_string())]
    DotIdent(String),

    #[token(":")]
    Colon,
    #[token(",")]
    Comma,

    #[regex(r"[rR]([0-9]|1[0-5])", |lex| lex.slice()[1..].parse().ok())]
    Register(u8),

    #[regex(r"0[xX][0-9a-fA-F]+", |lex| i64::from_str_radix(&lex.slice()[2..], 16).ok())] // Hexadecimal
    #[regex(r"0[oO][0-7]+", |lex| i64::from_str_radix(&lex.slice()[2..], 8).ok())] // Octal
    #[regex(r"0[bB][0-1]+", |lex| i64::from_str_radix(&lex.slice()[2..], 2).ok())] // Binary
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse().ok())] // Decimal/Denary
    Immediate(i64),

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum LexError {
    InvalidToken(usize),
}

// This is the bridge: lalrpop wants an Iterator<Item = Result<(usize, Token, usize), LexError>>
pub struct LexerAdapter<'input> {
    inner: logos::Lexer<'input, Token>,
}

impl<'input> LexerAdapter<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            inner: Token::lexer(input),
        }
    }
}

impl<'input> Iterator for LexerAdapter<'input> {
    type Item = Result<(usize, Token, usize), LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.inner.next()?;
        let span = self.inner.span();
        match token {
            Ok(tok) => Some(Ok((span.start, tok, span.end))),
            Err(_) => Some(Err(LexError::InvalidToken(span.start))),
        }
    }
}
