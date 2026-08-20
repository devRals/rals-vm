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
        bytes.get(start..end).expect(
            "Program counter got out of memory bounds. You might forgot to use HLT instruction.",
        )
    }

    pub fn decode(&self, raw_ins: &[u8]) -> Instruction<A> {
        Instruction::decode(raw_ins)
    }

    pub fn execute(&mut self, instruction: Instruction<A>) {
        use Instruction as I;

        match instruction {
            I::NOP => self.execute_nop(),

            I::INC { reg } => self.execute_inc(reg),
            I::DEC { reg } => self.execute_dec(reg),

            I::ADD { dst, lhs, rhs } => self.execute_alu_add(dst, lhs, rhs),
            I::SUB { dst, lhs, rhs } => self.execute_alu_sub(dst, lhs, rhs),
            I::OR { dst, lhs, rhs } => self.execute_alu_or(dst, lhs, rhs),
            I::XOR { dst, lhs, rhs } => self.execute_alu_xor(dst, lhs, rhs),
            I::AND { dst, lhs, rhs } => self.execute_alu_and(dst, lhs, rhs),
            I::SHL { dst, lhs, rhs } => self.execute_alu_shl(dst, lhs, rhs),
            I::SHR { dst, lhs, rhs } => self.execute_alu_shr(dst, lhs, rhs),
            I::SAR { dst, lhs, rhs } => self.execute_alu_sar(dst, lhs, rhs),

            I::ADDI { dst, lhs, imm } => self.execute_alu_addi(dst, lhs, imm),
            I::SUBI { dst, lhs, imm } => self.execute_alu_subi(dst, lhs, imm),
            I::ORI { dst, lhs, imm } => self.execute_alu_ori(dst, lhs, imm),
            I::XORI { dst, lhs, imm } => self.execute_alu_xori(dst, lhs, imm),
            I::ANDI { dst, lhs, imm } => self.execute_alu_andi(dst, lhs, imm),
            I::SHLI { dst, lhs, imm } => self.execute_alu_shli(dst, lhs, imm),
            I::SHRI { dst, lhs, imm } => self.execute_alu_shri(dst, lhs, imm),
            I::SARI { dst, lhs, imm } => self.execute_alu_sari(dst, lhs, imm),

            I::CMP { r1, r2 } => self.execute_alu_cmp(r1, r2),

            I::JO { addr } => self.execute_jo(addr),
            I::JS { addr } => self.execute_js(addr),
            I::JC { addr } => self.execute_jc(addr),
            I::JZ { addr } => self.execute_jz(addr),

            I::JNO { addr } => self.execute_jno(addr),
            I::JNS { addr } => self.execute_jns(addr),
            I::JNC { addr } => self.execute_jnc(addr),
            I::JNZ { addr } => self.execute_jnz(addr),

            I::JRO { amount } => self.execute_jro(amount),
            I::JRS { amount } => self.execute_jrs(amount),
            I::JRC { amount } => self.execute_jrc(amount),
            I::JRZ { amount } => self.execute_jrz(amount),

            I::JRNO { amount } => self.execute_jrno(amount),
            I::JRNS { amount } => self.execute_jrns(amount),
            I::JRNC { amount } => self.execute_jrnc(amount),
            I::JRNZ { amount } => self.execute_jrnz(amount),

            I::JMP { addr } => self.execute_jmp(addr),
            I::JMR { amount } => self.execute_jmp(amount),

            I::LDI { dst, src } => self.execute_ldi(dst, src),
            I::MOV { dst, src } => self.execute_mov(dst, src),

            I::LOAD {
                dst,
                target_base,
                target_index,
                target_displacement,
            } => self.execute_load(dst, target_base, target_index, target_displacement),
            I::STORE {
                target,
                dst_base,
                dst_index,
                dst_displacement,
            } => self.execute_store(dst_base, dst_index, dst_displacement, target),

            I::PUSH { .. } | I::POP { .. } => {
                panic!("stack pointer and its instructions are implemented yet")
            }

            I::UnknownInstruction => panic!(
                "rals-vm got an unknown instruction. This might be occured due decoding or encoding might be wrong for an object"
            ),
            I::HLT => self.execute_hlt(),
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
