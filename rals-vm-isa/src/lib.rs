//! ISA (Instruction Set Architecture) is an abstract model that defines the programmable
//! interface of the CPU of a computer, defining how software interacts with hardware.

pub mod arch;
pub mod instructions;
pub mod value;

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
