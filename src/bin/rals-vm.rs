use rals_vm::{
    cpu::isa::arch::Arch64,
    vm::{Program, VirtualMachine},
};

fn main() {
    let mut vm = VirtualMachine::<Arch64>::new([0; _]);

    let program = Program::<Arch64>::new(&[]);

    // consider [`Iterator::next`] is the fetch stage
    for raw_instruction in program {
        // Decode the instruction
        let ins = vm.decode(raw_instruction);
        // And execute it
        vm.execute(ins);
        // Increase the program counter for next instruction
        vm.cpu.reg_file.pc.advance();
    }
}
