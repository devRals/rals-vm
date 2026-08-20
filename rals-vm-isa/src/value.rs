use core::cmp::*;
use core::ops::*;
use std::fmt::Display;

use crate::Decode;

pub trait ImmediateValue:
    Copy
    + Display
    + Decode
    + Add<Output = Self>
    + Sub<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shr<Output = Self>
    + Shl<Output = Self>
    + Not<Output = Self>
    + PartialEq<Self>
    + PartialOrd<Self>
{
    type Bytes: AsRef<[u8]>;

    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    const SIGN_MASK: Self;
    const BITS: u32;
    const BYTES: usize;

    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;

    fn to_bytes(&self) -> Self::Bytes;
    /// Tries to cast the given value to [`ImmediateValue`]. Returns [`None`] if value is way of
    /// [`ImmediateValue`]'s bit range
    /// 8 bytes is the size rust uses to store (u/i)64 and (u/i)size values.
    /// Note that rals-vm uses less endian bytes for immediate values
    /// So using native or bigger endian would return diffirent values in the VM side
    fn try_from_signed(value: i64) -> Option<Self>;
    fn try_from_usize(value: usize) -> Option<Self>;
    fn as_usize(self) -> usize;
}

macro_rules! impl_integer_value {
    ($($ty:ty),*) => {
        $(
            impl ImmediateValue for $ty {
                type Bytes = [u8; <$ty>::BITS as usize / 8];

                const ZERO: Self = 0;
                const ONE: Self = 1;
                const BITS: u32 = <$ty>::BITS;
                const MAX: Self = <$ty>::MAX;
                const BYTES: usize = <$ty>::BITS as usize / 8;
                const SIGN_MASK: Self =
                    1 << (<$ty>::BITS - 1);

                fn overflowing_add(self, rhs: Self) -> (Self, bool) { self.overflowing_add(rhs) }
                fn overflowing_sub(self, rhs: Self) -> (Self, bool) { self.overflowing_sub(rhs) }

                fn wrapping_add(self, rhs: Self) -> Self {
                    self.wrapping_add(rhs)
                }
                fn wrapping_sub(self, rhs: Self) -> Self {
                    self.wrapping_sub(rhs)
                }

                fn to_bytes(&self) -> Self::Bytes {
                    self.to_le_bytes()
                }

                fn try_from_signed(value: i64) -> Option<Self> {
                    let bits = <$ty>::BITS;

                    if Self::BITS < i64::BITS {
                        let min = -(1i64 << (bits - 1));
                        let max = (1i64 << bits) - 1;

                        if value < min || value > max {
                            return None;
                        }
                    }

                    Some(value as $ty)
                }

                fn try_from_usize(value: usize) -> Option<Self> {
                    let bits = <$ty>::BITS;

                    if bits < usize::BITS {
                        let max = (1_usize << bits) - 1;

                        if value > max {
                            return None;
                        }
                    }

                    Some(value as $ty)
                }

                fn as_usize(self) -> usize { self as usize }
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

impl_integer_value!(u8, u16, u32, u64);
