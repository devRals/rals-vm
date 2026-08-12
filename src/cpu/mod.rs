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
    UnknownRegister { id: u8 },
    UnknownOpCode { code: u8 },
    InvalidLength { expected: usize, got: usize },
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownRegister { id } => write!(f, "unknown register {id}"),
            Self::UnknownOpCode { code } => write!(f, "unknown opcode {code}"),
            Self::InvalidLength { expected, got } => {
                write!(f, "Invalid length expected: {expected}, got: {got}")
            }
        }
    }
}
impl core::error::Error for DecodeError {}
