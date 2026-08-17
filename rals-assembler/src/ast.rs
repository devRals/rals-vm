pub struct AstProgram {
    pub sections: Vec<AstSection>,
}

pub struct AstSection {
    pub name: String,
    pub items: Vec<AstItem>,
}

pub enum AstItem {
    Label(String),
    Directive { key: String, value: i64 },
    Instruction(AstInstruction),
}

pub struct Directive {
    pub key: String,
    pub value: i64,
}

pub struct AstInstruction {
    pub mnemonic: String,
    pub operands: Vec<AstOperand>,
}

pub enum Instruction {
    NOP,

    INC {
        reg: u8,
    },
    DEC {
        reg: u8,
    },

    ADD {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },
    SUB {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },

    ADDI {
        dst: u8,
        lhs: u8,
        imm: i64,
    },
    SUBI {
        dst: u8,
        lhs: u8,
        imm: i64,
    },

    SHL {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },
    SHR {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },
    SAR {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },

    SHLI {
        dst: u8,
        lhs: u8,
        imm: i64,
    },
    SHRI {
        dst: u8,
        lhs: u8,
        imm: i64,
    },
    SARI {
        dst: u8,
        lhs: u8,
        imm: i64,
    },

    OR {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },
    XOR {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },
    AND {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },

    ORI {
        dst: u8,
        lhs: u8,
        imm: i64,
    },
    XORI {
        dst: u8,
        lhs: u8,
        imm: i64,
    },
    ANDI {
        dst: u8,
        lhs: u8,
        imm: i64,
    },

    CMP {
        r1: u8,
        r2: u8,
    },

    LDI {
        reg: u8,
        imm: i64,
    },
    MOV {
        r1: u8,
        r2: u8,
    },

    /// This instruction will be resolved in the pass2 stage
    JMPUnresolved {
        target: AstJumpTarget,
    },
    JMR {
        amount: i64,
    },

    /// This instruction will be resolved in the pass2 stage
    JOUnresolved {
        target: AstJumpTarget,
    },
    /// This instruction will be resolved in the pass2 stage
    JCUnresolved {
        target: AstJumpTarget,
    },
    /// This instruction will be resolved in the pass2 stage
    JZUnresolved {
        target: AstJumpTarget,
    },
    /// This instruction will be resolved in the pass2 stage
    JSUnresolved {
        target: AstJumpTarget,
    },

    /// This instruction will be resolved in the pass2 stage
    JNOUnresolved {
        target: AstJumpTarget,
    },
    /// This instruction will be resolved in the pass2 stage
    JNCUnresolved {
        target: AstJumpTarget,
    },
    /// This instruction will be resolved in the pass2 stage
    JNZUnresolved {
        target: AstJumpTarget,
    },
    /// This instruction will be resolved in the pass2 stage
    JNSUnresolved {
        target: AstJumpTarget,
    },

    JMP {
        addr: usize,
    },

    JO {
        addr: usize,
    },
    JC {
        addr: usize,
    },
    JZ {
        addr: usize,
    },
    JS {
        addr: usize,
    },

    JNO {
        addr: usize,
    },
    JNC {
        addr: usize,
    },
    JNZ {
        addr: usize,
    },
    JNS {
        addr: usize,
    },

    JRO {
        amount: i64,
    },
    JRC {
        amount: i64,
    },
    JRZ {
        amount: i64,
    },
    JRS {
        amount: i64,
    },

    JRNO {
        amount: i64,
    },
    JRNC {
        amount: i64,
    },
    JRNZ {
        amount: i64,
    },
    JRNS {
        amount: i64,
    },

    LOAD {
        dst: u8,
        target_base: u8,
        target_offset: i64,
    },
    STR {
        dst: u8,
        target_base: u8,
        target_offset: i64,
    },

    PUSH {
        reg: u8,
    },
    POP {
        reg: u8,
    },

    HLT,
}

pub enum AstJumpTarget {
    Label(String),
    Addr(usize),
}

#[derive(Clone)]
pub enum AstOperand {
    Reg(u8),
    Imm(i64),
    Label(String),
    Deref { base: u8, offset: i64 },
}

pub struct TextSection {
    pub instructions: Vec<Instruction>,
}

pub struct HeaderSection {
    pub directives: Vec<Directive>,
}
