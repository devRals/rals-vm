use crate::{
    cpu::{
        isa::{arch::Architecture, instructions::Immediate},
        reg_file::Register,
    },
    vm::VirtualMachine,
};

impl<A: Architecture> VirtualMachine<A> {
    pub const fn execute_nop(&mut self) {
        // What a cool instruction :3
    }

    pub fn execute_add(&mut self, lhs: Register, rhs: Register, dst: Register) {
        let lhs = self.cpu.reg_file.general[lhs as usize];
        let rhs = self.cpu.reg_file.general[rhs as usize];

        self.cpu.reg_file.general[dst as usize] = lhs + rhs;
    }

    pub fn execute_sub(&mut self, lhs: Register, rhs: Register, dst: Register) {
        let lhs = self.cpu.reg_file.general[lhs as usize];
        let rhs = self.cpu.reg_file.general[rhs as usize];

        self.cpu.reg_file.general[dst as usize] = lhs - rhs;
    }

    pub fn execute_ldi(&mut self, dst: Register, src: Immediate<A>) {
        let dst = &mut self.cpu.reg_file.general[dst as usize];
        *dst = src.value;
    }

    pub fn execute_mov(&mut self, dst: Register, src: Register) {
        self.cpu.reg_file.general[dst as usize] = self.cpu.reg_file.general[src as usize];
    }

    pub fn execute_hlt(&mut self) {
        self.cpu.reg_file.pc.hlt();
    }
}
