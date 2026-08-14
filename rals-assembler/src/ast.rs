#[derive(Debug)]
pub struct Program {
    pub sections: Vec<Section>,
}

#[derive(Debug)]
pub struct Section {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Label(String),
    Directive { key: String, value: i64 },
    Instruction(Instruction),
}

#[derive(Debug)]
pub struct Instruction {
    pub mnemonic: String,
    pub operands: Vec<Operand>,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Reg(u8),
    Imm(i64),
    Label(String),
}
