use rals_vm::{cpu::isa::arch::Arch8, vm::VirtualMachine};

type Arch = Arch8;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let mut vm = VirtualMachine::<Arch>::new([0; _]);

    let program = [
        // ._start:
        [0x5, 0x1, 5, 0],      // LDI R1, 5
        [0x5, 0x2, 7, 0],      // LDI R2, 7
        [0x2, 0x1, 0x2, 0x3],  // SUB R1, R2, R3
        [0xff, 0x0, 0x0, 0x0], // HLT
    ]
    .concat();

    vm.set_mem(&*program);

    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?; // After the HLT machine stops executing instructions
    vm.step()?; // After the HLT machine stops executing instructions
    vm.step()?; // After the HLT machine stops executing instructions
    vm.step()?; // After the HLT machine stops executing instructions

    println!("registers: {:#?}", vm.cpu.reg_file.general);

    Ok(())
}
