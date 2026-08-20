use crate::vm::VirtualMachine;
use rals_vm_isa::{
    Decode,
    arch::{Architecture, AsBytes},
    registers::Register,
    value::ImmediateValue,
};

macro_rules! execute_method {
    (alu $name: ident -> $alu_action: ident) => {
        pub fn $name(&mut self, dst: Register, lhs: Register, rhs: Register) {
            let lhs = self.cpu.reg_file.general[lhs as usize];
            let rhs = self.cpu.reg_file.general[rhs as usize];

            let result = self.cpu.alu.$alu_action(lhs, rhs);

            self.cpu.reg_file.set_reg(dst, result.value);
            self.cpu.flags = result.flags;
        }
    };

    (alu_imm $name: ident -> $alu_action: ident) => {
        pub fn $name(&mut self, dst: Register, lhs: Register, imm: A::Word) {
            let lhs = self.cpu.reg_file.general[lhs as usize];
            let result = self.cpu.alu.$alu_action(lhs, imm);

            self.cpu.reg_file.set_reg(dst, result.value);
            self.cpu.flags = result.flags;
        }
    };

    ($name: ident -> jump if $flag: ident) => {
        pub fn $name(&mut self, addr: A::Word) {
            if self.cpu.flags.$flag {
                self.cpu.reg_file.pc.jmp(addr)
            }
        }
    };
    ($name: ident -> jump if not $flag: ident) => {
        pub fn $name(&mut self, addr: A::Word) {
            if !self.cpu.flags.$flag {
                self.cpu.reg_file.pc.jmp(addr)
            }
        }
    };
    ($name: ident -> jump relative if $flag: ident) => {
        pub fn $name(&mut self, amount: A::Word) {
            if self.cpu.flags.$flag {
                self.cpu.reg_file.pc.jmr(amount)
            }
        }
    };
    ($name: ident -> jump relative if not $flag: ident) => {
        pub fn $name(&mut self, amount: A::Word) {
            if !self.cpu.flags.$flag {
                self.cpu.reg_file.pc.jmr(amount)
            }
        }
    };
}

impl<A: Architecture> VirtualMachine<A> {
    pub fn execute_nop(&mut self) { /* What a cool instruction :3 */
    }

    pub fn execute_inc(&mut self, reg: Register) {
        self.cpu.reg_file.inc(reg);
    }
    pub fn execute_dec(&mut self, reg: Register) {
        self.cpu.reg_file.dec(reg);
    }

    pub fn execute_ldi(&mut self, dst: Register, src: A::Word) {
        self.cpu.reg_file.set_reg(dst, src);
    }

    pub fn execute_mov(&mut self, dst: Register, src: Register) {
        self.cpu
            .reg_file
            .set_reg(dst, self.cpu.reg_file.general[src as usize]);
    }

    execute_method!(alu execute_alu_add -> add);
    execute_method!(alu execute_alu_sub -> sub);
    execute_method!(alu execute_alu_or  -> or);
    execute_method!(alu execute_alu_xor -> xor);
    execute_method!(alu execute_alu_and -> and);
    execute_method!(alu execute_alu_shl -> shl);
    execute_method!(alu execute_alu_shr -> shr);
    execute_method!(alu execute_alu_sar -> sar);

    execute_method!(alu_imm execute_alu_addi -> add);
    execute_method!(alu_imm execute_alu_subi -> sub);
    execute_method!(alu_imm execute_alu_ori  -> or);
    execute_method!(alu_imm execute_alu_xori -> xor);
    execute_method!(alu_imm execute_alu_andi -> and);
    execute_method!(alu_imm execute_alu_shli -> shl);
    execute_method!(alu_imm execute_alu_shri -> shr);
    execute_method!(alu_imm execute_alu_sari -> sar);

    pub fn execute_alu_cmp(&mut self, r1: Register, r2: Register) {
        let a = self.cpu.reg_file.general[r1 as usize];
        let b = self.cpu.reg_file.general[r2 as usize];

        self.cpu.flags = self.cpu.alu.sub(a, b).flags
    }

    execute_method!(execute_jo -> jump if overflow);
    execute_method!(execute_jc -> jump if carry);
    execute_method!(execute_js -> jump if sign);
    execute_method!(execute_jz -> jump if zero);

    execute_method!(execute_jno -> jump if not overflow);
    execute_method!(execute_jnc -> jump if not carry);
    execute_method!(execute_jns -> jump if not sign);
    execute_method!(execute_jnz -> jump if not zero);

    execute_method!(execute_jro -> jump relative if overflow);
    execute_method!(execute_jrc -> jump relative if carry);
    execute_method!(execute_jrs -> jump relative if sign);
    execute_method!(execute_jrz -> jump relative if zero);

    execute_method!(execute_jrno -> jump relative if not overflow);
    execute_method!(execute_jrnc -> jump relative if not carry);
    execute_method!(execute_jrns -> jump relative if not sign);
    execute_method!(execute_jrnz -> jump relative if not zero);

    pub fn execute_jmp(&mut self, addr: A::Word) {
        self.cpu.reg_file.pc.jmp(addr);
    }
    pub fn execute_jmr(&mut self, amount: A::Word) {
        self.cpu.reg_file.pc.jmr(amount);
    }

    pub fn execute_load(
        &mut self,
        dst: Register,
        target_base: Register,
        target_index: Register,
        target_displacement: A::Word,
    ) {
        let base = self.cpu.reg_file.general[target_base as usize];
        let index = self.cpu.reg_file.general[target_index as usize];
        let displacement = target_displacement;

        let address = base
            .wrapping_add(index)
            .wrapping_add(displacement)
            .as_usize();

        let bytes = &self.mem.data.as_bytes()[address..address + A::Word::BYTES];
        let target_value = A::Word::decode(bytes);
        self.cpu.reg_file.set_reg(dst, target_value);
    }

    pub fn execute_store(
        &mut self,
        dst_base: Register,
        dst_index: Register,
        dst_displacement: A::Word,
        target: Register,
    ) {
        let base = self.cpu.reg_file.general[dst_base as usize];
        let index = self.cpu.reg_file.general[dst_index as usize];
        let displacement = dst_displacement;

        let dst_address = base
            .wrapping_add(index)
            .wrapping_add(displacement)
            .as_usize();

        let target_value = self.cpu.reg_file.general[target as usize];
        let target_value_bytes = target_value.to_bytes();

        let memory = self.mem.data.as_bytes_mut();
        memory[dst_address..dst_address + A::Word::BYTES]
            .copy_from_slice(target_value_bytes.as_ref());
    }

    pub fn execute_hlt(&mut self) {
        self.cpu.reg_file.pc.hlt();
    }
}
