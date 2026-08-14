use core::cmp::*;
use core::ops::*;

use crate::{Decode, DecodeError, Encode, instructions::Operand};

pub trait ImmediateValue:
    Copy
    + Operand
    + Add<Output = Self>
    + Sub<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shr<u32, Output = Self>
    + Shl<u32, Output = Self>
    + Not<Output = Self>
    + PartialEq<Self>
    + PartialOrd<Self>
{
    type Bytes: AsRef<[u8]>;

    const ZERO: Self;
    const ONE: Self;
    const SIGN_MASK: Self;
    const BITS: u32;
    const BYTES: usize;

    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);

    fn to_bytes(&self) -> Self::Bytes;
}

macro_rules! impl_integer_value {
    ($($ty:ty),*) => {
        $(
            impl ImmediateValue for $ty {
                type Bytes = [u8; <$ty>::BITS as usize / 8];

                const ZERO: Self = 0;
                const ONE: Self = 1;
                const BITS: u32 = <$ty>::BITS;
                const BYTES: usize = <$ty>::BITS as usize / 8;
                const SIGN_MASK: Self =
                    1 << (<$ty>::BITS - 1);

                fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                    self.overflowing_add(rhs)
                }

                fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                    self.overflowing_sub(rhs)
                }

                fn to_bytes(&self) -> Self::Bytes {
                    self.to_le_bytes()
                }
            }

            impl Operand for $ty {}

            impl Encode for $ty {
                fn encode(self, out: &mut [u8]) {
                    out.copy_from_slice(&self.to_le_bytes());
                }
            }

            impl Decode for $ty {
                fn decode(ins: &[u8]) -> Result<Self, DecodeError> {
                    let bytes: [u8; Self::BYTES] = ins[..Self::BYTES]
                        .try_into()
                        .map_err(|_| DecodeError::InvalidLength {
                            expected: Self::BYTES,
                            got: ins.len(),
                        })?;

                    Ok(Self::from_le_bytes(bytes))
                }
            }
        )*
    };
}

impl_integer_value!(u8, u16, u32, u64, u128);

pub fn shl_carry<T: ImmediateValue>(value: T, amount: u32) -> bool {
    if amount == 0 {
        return false;
    }

    ((value >> (T::BITS - amount)) & T::ONE) != T::ZERO
}

pub fn shr_carry<T: ImmediateValue>(value: T, amount: u32) -> bool {
    if amount == 0 {
        return false;
    }

    ((value >> (amount - 1)) & T::ONE) != T::ZERO
}
