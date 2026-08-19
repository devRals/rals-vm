mod execution;

use crate::{cpu::CentralProcessUnit, ram::RandomAccessMemory};

use rals_vm_isa::{
    Decode,
    arch::{Arch64, Architecture, AsBytes},
    instructions::Instruction,
};

pub struct VirtualMachine<A: Architecture = Arch64> {
    pub cpu: CentralProcessUnit<A>,
    pub mem: RandomAccessMemory<A>,
}

impl<A: Architecture> VirtualMachine<A> {
    pub fn new() -> Self {
        VirtualMachine {
            cpu: CentralProcessUnit::new(),
            mem: RandomAccessMemory {
                data: A::new_empty_memory(),
            },
        }
    }

    /// WARNING! Completely changes the vm's memory from top to bottom with the given bytes and fills the empty
    /// spaces with 0's'
    /// Consider using [`Self::load_instructions`] if you don't know what you're doing
    pub fn set_mem<'a>(&'a mut self, bytes: impl Into<&'a [u8]>) {
        let bytes: &[u8] = bytes.into();
        let mem = self.mem.data.as_bytes_mut();

        if bytes.len() > mem.len() {
            panic!("bytes len passes the mem capacity")
        }

        mem.fill(0);
        for (src, dst) in bytes.iter().zip(mem) {
            *dst = *src;
        }
    }

    pub fn load_instructions<I>(&mut self, instructions: I)
    where
        I: IntoIterator<Item = A::Instruction>,
    {
        let mem = self.mem.data.as_bytes_mut();
        for (i, ins) in instructions.into_iter().enumerate() {
            let chunk = &mut mem[i * A::INSTRUCTION_SIZE..(i + 1) * A::INSTRUCTION_SIZE];
            chunk.copy_from_slice(ins.as_bytes());
        }
    }

    pub fn fetch(&self) -> &[u8] {
        let pc = &self.cpu.reg_file.pc;

        let start = pc.get() as usize;
        let end = start + A::INSTRUCTION_SIZE;

        let bytes = self.mem.data.as_bytes();
        &bytes[start..end]
    }

    pub fn decode(&self, raw_ins: &[u8]) -> Instruction<A> {
        Instruction::decode(raw_ins)
    }

    pub fn execute(&mut self, instruction: Instruction<A>) {
        use Instruction as I;

        match instruction {
            I::NOP => self.execute_nop(),

            I::ADD { dst, lhs, rhs } => self.execute_add(lhs, rhs, dst),
            I::SUB { dst, lhs, rhs } => self.execute_sub(lhs, rhs, dst),

            I::LDI { dst, src } => self.execute_ldi(dst, src),
            I::MOV { dst, src } => self.execute_mov(dst, src),

            I::HLT => self.execute_hlt(),
            _ => todo!("this instruction not implemented yet"),
        }
    }

    pub fn step(&mut self) {
        let raw_instruction = self.fetch();

        let instruction = self.decode(raw_instruction);
        self.execute(instruction);

        self.cpu.reg_file.pc.advance();
    }

    /// Keeps executing the loaded instructions until the system reaches an HLT instruction
    pub fn run(&mut self) {
        while !self.cpu.reg_file.pc.stopped {
            self.step();
        }
    }

    pub fn fail(&self, msg: &str) -> ! {
        panic!("Segmentation fault noooo :c\n    {}", msg)
    }
}
