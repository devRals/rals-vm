use core::marker::PhantomData;

use crate::cpu::{
    isa::arch::{Arch8, Arch16, Arch32, Arch64, Architecture},
    isa::value::{ImmediateValue, shl_carry, shr_carry},
};

/// # ALU (Arithmetic Logic Unit)
/// ALU is a physical&hardware CPU component that capable of
/// doing arithmetic operations by just using raw electric signals. Since our VM
/// does not need electric signals we can use rust's built-in operations
///
/// A simple ALU normally recieves three inputs, gives one output as result and sets returns couple
/// flags depending on the resulted value.. First input is the OpCode (Operation Code) which defines
/// what operation should ALU operate. Other the are the operends which are just
/// values that will be calculated using the recieved operation type and will be returned as output.
pub struct ArithmeticLogicUnit<A: Architecture> {
    _arch: PhantomData<A>,
}

impl<A: Architecture> ArithmeticLogicUnit<A> {
    pub const fn new() -> Self {
        ArithmeticLogicUnit { _arch: PhantomData }
    }
}

pub type Alu8 = ArithmeticLogicUnit<Arch8>;
pub type Alu16 = ArithmeticLogicUnit<Arch16>;
pub type Alu32 = ArithmeticLogicUnit<Arch32>;
pub type Alu64 = ArithmeticLogicUnit<Arch64>;

#[derive(Clone, Copy)]
pub struct Flags {
    /// Result is zero
    pub zero: bool,
    /// Result is negative
    pub sign: bool,
    /// Result overflowed it's bit size
    pub overflow: bool,
    /// Result has a carry bit
    pub carry: bool,
}

fn flags<T: ImmediateValue>(result: T, carry: bool, overflow: bool) -> Flags {
    Flags {
        zero: result == T::ZERO,
        sign: (result & T::SIGN_MASK) != T::ZERO,
        carry,
        overflow,
    }
}

pub struct AluResult<T> {
    pub value: T,
    pub flags: Flags,
}

impl<T: ImmediateValue> AluResult<T> {
    fn new(value: T, carry: bool, overflow: bool) -> Self {
        Self {
            value,
            flags: flags(value, carry, overflow),
        }
    }
}

macro_rules! impl_alu {
    (
        $(
            $op_name:ident($($param:ident: $param_ty: ty),*) ->
                $value: ident: $action:expr,
                overflow: $overflow:expr
        )*
    ) => {
        impl<A: Architecture> ArithmeticLogicUnit<A> {
            $(
                pub fn $op_name(
                    &self,
                    $($param: $param_ty),*
                ) -> AluResult<A::Word> {

                    let ($value, carry) = $action;

                    let overflow = $overflow;

                    AluResult::new($value, carry, overflow)
                }
            )*
        }
    };
}

impl_alu! {
    add(a: A::Word, b: A::Word) ->
        value: a.overflowing_add(b),
        overflow: ((a ^ value) & (b ^ value) & A::SIGN_MASK) != A::Word::ZERO
    sub(a: A::Word, b: A::Word) ->
        value: a.overflowing_sub(b),
        overflow: ((a ^ b) & (a ^ value) & A::SIGN_MASK) != A::Word::ZERO

    or(a: A::Word, b: A::Word) ->
        value: (a | b, false),
        overflow: false
    xor(a: A::Word, b: A::Word) ->
        value: (a ^ b, false),
        overflow: false
    and(a: A::Word, b: A::Word) ->
        value: (a & b, false),
        overflow: false

    shl(a: A::Word, amount: u32) ->
        value: {
            let value = a << amount;
            let carry = shl_carry(a, amount);
            (value, carry)
        },
        overflow: {
            let old_sign = (a & A::SIGN_MASK) != A::Word::ZERO;
            let new_sign = (value & A::SIGN_MASK) != A::Word::ZERO;

            let overflow = amount == 1 && old_sign != new_sign;
            overflow
        }
    shr(a: A::Word, amount: u32) ->
        value: {
            let value = a >> amount;
            let carry = shr_carry(a, amount);
            (value, carry)
        },
        overflow: amount == 1 && (a & A::SIGN_MASK) != A::Word::ZERO

    inv(a: A::Word) ->
        value: (!a, false),
        overflow: false
}

impl<A: Architecture> ArithmeticLogicUnit<A> {}
