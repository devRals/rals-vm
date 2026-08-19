//! ISA (Instruction Set Architecture) is an abstract model that defines the programmable
//! interface of the CPU of a computer, defining how software interacts with hardware.

pub mod arch;
pub mod instructions;
pub mod registers;
pub mod value;

pub trait Encode {
    /// In a real application return value should be wrapped in a Result enum
    /// but since we're just trying to replicate a real cpu this is not required
    fn encode(self, out: &mut [u8]);
}

pub trait Decode {
    /// In a real application return value should be wrapped in a Result enum
    /// but since we're just trying to replicate a real cpu this is not required
    fn decode(ins: &[u8]) -> Self
    where
        Self: Sized;
}
