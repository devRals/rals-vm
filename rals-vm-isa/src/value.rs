use core::cmp::*;
use core::ops::*;

use crate::Decode;

pub trait ImmediateValue:
    Copy
    + Decode
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
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;

    fn to_bytes(&self) -> Self::Bytes;
    fn try_from_i64(value: i64) -> Option<Self>; // For assembly side interplation
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

                fn wrapping_add(self, rhs: Self) -> Self {
                    self.wrapping_add(rhs)
                }
                fn wrapping_sub(self, rhs: Self) -> Self {
                    self.wrapping_sub(rhs)
                }

                fn to_bytes(&self) -> Self::Bytes {
                    self.to_le_bytes()
                }

                fn try_from_i64(value: i64) -> Option<Self> {
                    Self::try_from(value).ok()
                }
            }

            impl Decode for $ty {
                fn decode(ins: &[u8]) -> Self {
                    let bytes: [u8; Self::BYTES] = ins[..Self::BYTES]
                        .try_into()
                        .expect("Immediate value decoding design is wrong. Update your code");

                    Self::from_le_bytes(bytes)
                }
            }
        )*
    };
}

impl_integer_value!(u8, u16, u32, u64, u128);
