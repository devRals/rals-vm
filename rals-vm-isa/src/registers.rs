use crate::Decode;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    RSP,
    RFP,

    UnknownRegister = 0xFF,
}

impl Decode for Register {
    fn decode(ins: &[u8]) -> Self {
        let byte = *ins
            .first()
            .expect("Register decode design is wrong. Update your code");

        match byte {
            0x00 => Register::R0,
            0x01 => Register::R1,
            0x02 => Register::R2,
            0x03 => Register::R3,
            0x04 => Register::R4,
            0x05 => Register::R5,
            0x06 => Register::R6,
            0x07 => Register::R7,
            0x08 => Register::R8,
            0x09 => Register::R9,
            0x0a => Register::R10,
            0x0b => Register::R11,
            0x0c => Register::R12,
            0x0d => Register::R13,
            0x0e => Register::R14,
            0x0f => Register::R15,

            0x10 => Register::RSP,
            0x11 => Register::RFP,

            _ => Register::UnknownRegister,
        }
    }
}
