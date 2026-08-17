#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: usize,
    pub len: usize,
}

impl Span {
    pub const ZERO: Self = Span { start: 0, len: 0 };
}

#[derive(Clone, Copy, Debug)]
pub struct SourcePos {
    pub line: u32,
    pub column: u32,
}

impl SourcePos {
    pub const ZERO: Self = SourcePos { line: 0, column: 0 };
}

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub ty: TokenType,
    pub pos: SourcePos,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    EOF,
    /// Ex: "?", "-", "Ğ"
    Illegal,
    Ident,
    /// Ex: 0xE2D0, 15, 0b100101, 0o755
    Constant,

    // # Keywords
    LDI,
    MOV,
    ADD,
    SUB,
    AND,
    OR,
    XOR,
    INV,
    SHL,
    SHR,

    CMP,
    JMP,
    JMR,

    JZ,
    JS,
    JO,
    JC,

    CALL,
    RET,

    Section,

    // # Signs
    /// ","
    Comma,
    /// "."
    Dot,
    /// ":"
    Colon,
}

impl Token {
    pub const fn new(ty: TokenType, pos: SourcePos, span: Span) -> Token {
        Self { ty, pos, span }
    }
}

impl Token {
    pub const EOF: Token = Token::new(TokenType::EOF, SourcePos::ZERO, Span::ZERO);

    pub const DUMMY_TOKEN: Token = Token::new(TokenType::Illegal, SourcePos::ZERO, Span::ZERO);

    pub fn literal<'a>(&self, source_str: &'a str) -> &'a str {
        &source_str[self.span.start..self.span.start + self.span.len]
    }

    pub fn keyword(literal: &str, pos: SourcePos, start: usize) -> Token {
        use TokenType::*;

        let (ty, len) = match literal {
            "ldi" | "LDI" => (LDI, 3),
            "mov" | "MOV" => (MOV, 3),

            "add" | "ADD" => (ADD, 3),
            "sub" | "SUB" => (SUB, 3),
            "or" | "OR" => (OR, 2),
            "xor" | "XOR" => (XOR, 3),
            "inv" | "INV" => (INV, 3),
            "shl" | "SHL" => (SHL, 3),
            "shr" | "SHR" => (SHR, 3),

            "cmp" | "CMP" => (CMP, 3),
            "jmp" | "JMP" | "jm" | "JM" => (JMP, 3),
            "jmr" | "JMR" | "jr" | "JR" => (JMR, 3),

            "jz" | "JZ" => (JZ, 3),
            "js" | "JS" => (JS, 3),
            "jo" | "JO" => (JO, 3),
            "jc" | "JC" => (JC, 3),

            "call" | "CALL" => (CALL, 3),
            "cll" | "CLL" => (CALL, 4),

            "ret" | "RET" => (RET, 3),

            "section" | "SECTION" => (Section, 7),
            l => (Ident, l.len()),
        };

        Token::new(ty, pos, Span { start, len })
    }
}
