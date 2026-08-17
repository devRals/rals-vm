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
        reg: u8,
        lhs: u8,
        rhs: u8,
    },
    SHR {
        reg: u8,
        lhs: u8,
        rhs: u8,
    },
    SAR {
        reg: u8,
        lhs: u8,
        rhs: u8,
    },

    SHLI {
        reg: u8,
        lhs: u8,
        imm: i64,
    },
    SHRI {
        reg: u8,
        lhs: u8,
        imm: i64,
    },
    SARI {
        reg: u8,
        lhs: u8,
        imm: i64,
    },

    OR {
        reg: u8,
        lhs: u8,
        rhs: u8,
    },
    XOR {
        reg: u8,
        lhs: u8,
        rhs: u8,
    },
    AND {
        reg: u8,
        lhs: u8,
        rhs: u8,
    },

    ORI {
        reg: u8,
        lhs: u8,
        imm: i64,
    },
    XORI {
        reg: u8,
        lhs: u8,
        imm: i64,
    },
    ANDI {
        reg: u8,
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

    JO {
        addr: u64,
    },
    JC {
        addr: u64,
    },
    JZ {
        addr: u64,
    },
    JS {
        addr: u64,
    },

    JNO {
        addr: u64,
    },
    JNC {
        addr: u64,
    },
    JNZ {
        addr: u64,
    },
    JNS {
        addr: u64,
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

    HLT,
}

pub enum AstJumpTarget {
    Label(String),
    Addr(u64),
    Imm(i64),
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
