use rals_vm_isa::arch::Architecture;

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

#[derive(Clone)]
pub enum Instruction<A: Architecture> {
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
        imm: A::Word,
    },
    SUBI {
        dst: u8,
        lhs: u8,
        imm: A::Word,
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
        imm: A::Word,
    },
    SHRI {
        dst: u8,
        lhs: u8,
        imm: A::Word,
    },
    SARI {
        dst: u8,
        lhs: u8,
        imm: A::Word,
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
        imm: A::Word,
    },
    XORI {
        dst: u8,
        lhs: u8,
        imm: A::Word,
    },
    ANDI {
        dst: u8,
        lhs: u8,
        imm: A::Word,
    },

    CMP {
        r1: u8,
        r2: u8,
    },

    LDI {
        reg: u8,
        imm: A::Word,
    },
    MOV {
        r1: u8,
        r2: u8,
    },

    /// This instruction will be resolved in the pass2 stage
    JMPUnresolved {
        target: AstJumpTarget<A>,
    },
    JMR {
        amount: A::Word,
    },

    /// This instruction will be resolved in the pass2 stage
    JOUnresolved {
        target: AstJumpTarget<A>,
    },
    /// This instruction will be resolved in the pass2 stage
    JCUnresolved {
        target: AstJumpTarget<A>,
    },
    /// This instruction will be resolved in the pass2 stage
    JZUnresolved {
        target: AstJumpTarget<A>,
    },
    /// This instruction will be resolved in the pass2 stage
    JSUnresolved {
        target: AstJumpTarget<A>,
    },

    /// This instruction will be resolved in the pass2 stage
    JNOUnresolved {
        target: AstJumpTarget<A>,
    },
    /// This instruction will be resolved in the pass2 stage
    JNCUnresolved {
        target: AstJumpTarget<A>,
    },
    /// This instruction will be resolved in the pass2 stage
    JNZUnresolved {
        target: AstJumpTarget<A>,
    },
    /// This instruction will be resolved in the pass2 stage
    JNSUnresolved {
        target: AstJumpTarget<A>,
    },

    JMP {
        addr: A::Word,
    },

    JO {
        addr: A::Word,
    },
    JC {
        addr: A::Word,
    },
    JZ {
        addr: A::Word,
    },
    JS {
        addr: A::Word,
    },

    JNO {
        addr: A::Word,
    },
    JNC {
        addr: A::Word,
    },
    JNZ {
        addr: A::Word,
    },
    JNS {
        addr: A::Word,
    },

    JRO {
        amount: A::Word,
    },
    JRC {
        amount: A::Word,
    },
    JRZ {
        amount: A::Word,
    },
    JRS {
        amount: A::Word,
    },

    JRNO {
        amount: A::Word,
    },
    JRNC {
        amount: A::Word,
    },
    JRNZ {
        amount: A::Word,
    },
    JRNS {
        amount: A::Word,
    },

    LOAD {
        dst: u8,
        target_base: u8,
        target_index: u8,
        target_displacement: A::Word,
    },
    STORE {
        dst_base: u8,
        dst_index: u8,
        dst_displacement: A::Word,
        target: u8,
    },

    PUSH {
        reg: u8,
    },
    POP {
        reg: u8,
    },

    HLT,
}

#[derive(Clone)]
pub enum AstJumpTarget<A: Architecture> {
    Label(String),
    Addr(A::Word),
}

#[derive(Clone)]
pub enum AstOperand {
    Reg(u8),
    Imm(i64),
    Label(String),
    Deref {
        base: u8,
        index: u8,
        displacement: i64,
    },
}

pub struct TextSection<A: Architecture> {
    pub instructions: Vec<Instruction<A>>,
}

pub struct HeaderSection {
    pub directives: Vec<Directive>,
}
