use crate::{
    cpu::{
        CentralProcessUnit, Decode,
        isa::{
            arch::{Arch64, Architecture, AsBytes},
            instructions::{Instruction, RawInstruction},
        },
    },
    ram::RandomAccessMemory,
};

pub struct VirtualMachine<A: Architecture = Arch64> {
    pub cpu: CentralProcessUnit<A>,
    pub mem: RandomAccessMemory<A>,
}

impl<A: Architecture> VirtualMachine<A> {
    pub const fn new(memory: A::Memory) -> Self {
        VirtualMachine {
            cpu: CentralProcessUnit::new(),
            mem: RandomAccessMemory { data: memory },
        }
    }

    pub fn load_instructions(&mut self, ins: RawInstruction<A>) {
        self.mem
            .data
            .as_bytes_mut()
            .copy_from_slice(ins.data.as_bytes());
    }

    pub fn fetch(&self) -> &[u8] {
        let bytes = self.mem.data.as_bytes();
        &bytes[self.cpu.reg_file.pc.get() as usize..A::INSTRUCTION_SIZE as usize]
    }

    pub fn decode(&self, raw_ins: &[u8]) -> Instruction<A> {
        match Instruction::decode(raw_ins) {
            Ok(ins) => ins,
            Err(_) => self.fail("Failed to decode instruction"),
        }
    }

    pub fn execute(&mut self, instruction: Instruction<A>) {
        match instruction {
            _ => {}
        }
    }

    pub fn step(&mut self) {
        let raw_instruction = self.fetch();

        let instruction = self.decode(raw_instruction);
        self.execute(instruction);

        self.cpu.reg_file.pc.advance();
    }

    pub fn fail(&self, msg: &str) -> ! {
        panic!("Segmentation fault noooo :c\n    {}", msg)
    }
}

pub struct Program<'ins, A: Architecture> {
    instructions: &'ins [A::Instruction],
    index: usize,
}

impl<'a, A: Architecture> Program<'a, A> {
    pub const fn new(instructions: &'a [A::Instruction]) -> Self {
        Program {
            instructions,
            index: 0,
        }
    }
}

impl<'a, A: Architecture> Iterator for Program<'a, A> {
    type Item = &'a A::Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let ins = self.instructions.get(self.index);
        self.index += 1;
        ins
    }
}
