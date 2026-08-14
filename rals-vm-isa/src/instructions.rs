use crate::{Decode, DecodeError, Encode, arch::Architecture, value::ImmediateValue};

pub trait Operand: Encode + Decode {}

pub struct Immediate<A: Architecture> {
    pub value: A::Word,
}

impl<A: Architecture> Encode for Immediate<A> {
    fn encode(self, out: &mut [u8]) {
        out[..A::Word::BYTES].copy_from_slice(self.value.to_bytes().as_ref());
    }
}

impl<A: Architecture> Decode for Immediate<A> {
    fn decode(ins: &[u8]) -> Result<Self, DecodeError> {
        Ok(Immediate {
            value: A::Word::decode(ins)?,
        })
    }
}

impl<A: Architecture> Operand for Immediate<A> {}

pub struct RawInstruction<A: Architecture> {
    pub data: A::Instruction,
}

impl<A: Architecture> RawInstruction<A> {
    pub const fn new(ins: A::Instruction) -> Self {
        RawInstruction { data: ins }
    }
}
