pub mod alu;
pub mod reg_file;

use alu::*;
use rals_vm_isa::arch::*;
use reg_file::*;

/// CPU is one of the most important components of a computer. It's responsible of doing
/// all of the arithmetic operations, executing given instructions and operating the
/// whole system
pub struct CentralProcessUnit<Arch: Architecture> {
    /// ALU is the component responsible for performing arithmetic operations
    pub alu: ArithmeticLogicUnit<Arch>,
    /// Register File is responsible for telling us which register holds which value. As an extra it
    /// also has a Program Counter that tells us which instruction we currently executing
    pub reg_file: RegisterFile<Arch>,
    /// CPU Flags are changes after every operation ALU does. Those are usually used by the later
    /// opeerations have "conditional branching".
    pub flags: Flags,
}

impl<A: Architecture> CentralProcessUnit<A> {
    pub const fn new() -> Self {
        CentralProcessUnit {
            alu: ArithmeticLogicUnit::new(),
            reg_file: RegisterFile::new(),
            flags: Flags {
                zero: false,
                sign: false,
                overflow: false,
                carry: false,
            },
        }
    }

    pub fn reset(&mut self) {
        self.reg_file.reset();
    }
}
