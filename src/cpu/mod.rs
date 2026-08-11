pub mod alu;
pub mod isa;
pub mod reg_file;

use alu::*;
use isa::arch::*;
use reg_file::*;

/// CPU is one of the most important components of a computer. It's responsible of doing
/// all of the arithmetic operations, executing given instructions and operating the
/// whole system
pub struct CentralProcessUnit<Arch: Architecture> {
    /// ALU is the component responsible for performing arithmetic operations
    pub alu: ArithmeticLogicUnit<Arch>,
    /// PC is responsible for telling us which instruction were currently executing
    pub reg_file: RegisterFile<Arch>,
}

impl<A: Architecture> CentralProcessUnit<A> {
    pub const fn new() -> Self {
        CentralProcessUnit {
            alu: ArithmeticLogicUnit::new(),
            reg_file: RegisterFile::new(),
        }
    }

    pub fn reset(&mut self) {
        self.reg_file.reset();
    }
}

pub trait Encode {
    fn encode(self, out: &mut [u8]);
}

pub trait Decode {
    fn decode(ins: &[u8]) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum DecodeError {
    UnknownRegister,
    UnknownOpCode,
    InvalidLength,
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnknownRegister => "tried to use an unknown register",
                Self::UnknownOpCode => "tried to use an unknown opcode",
                Self::InvalidLength => "Slice does not have enough space",
            }
        )
    }
}
impl core::error::Error for DecodeError {}
