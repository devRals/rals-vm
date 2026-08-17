use crate::{Decode, Encode, Operand};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

    UnknownRegister = 0xFF,
}

impl Operand for Register {}

impl Encode for Register {
    fn encode(self, out: &mut [u8]) {
        out[0] = self as u8;
    }
}

impl Decode for Register {
    fn decode(ins: &[u8]) -> Self {
        let byte = *ins
            .first()
            .expect("Register decode design is wrong. Update your code");

        match byte {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::R8,
            9 => Register::R9,
            10 => Register::R10,
            11 => Register::R11,
            12 => Register::R12,
            13 => Register::R13,
            14 => Register::R14,
            15 => Register::R15,
            _ => Register::UnknownRegister,
        }
    }
}
