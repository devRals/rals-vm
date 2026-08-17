use core::marker::PhantomData;

use rals_vm_isa::{
    arch::{Arch8, Arch16, Arch32, Arch64, Architecture},
    value::ImmediateValue,
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

fn shl_carry<T: ImmediateValue>(value: T, amount: u32) -> bool {
    if amount == 0 {
        return false;
    }

    ((value >> (T::BITS - amount)) & T::ONE) != T::ZERO
}

fn shr_carry<T: ImmediateValue>(value: T, amount: u32) -> bool {
    if amount == 0 {
        return false;
    }

    ((value >> (amount - 1)) & T::ONE) != T::ZERO
}

fn sar_carry<T: ImmediateValue>(value: T, amount: u32) -> bool {
    // same bit falls out as a logical shift right — carry doesn't care about sign-fill
    shr_carry(value, amount)
}

fn sar_value<T: ImmediateValue>(value: T, amount: u32) -> T {
    if amount == 0 {
        return value;
    }

    let shifted = value >> amount;
    let sign_set = (value & T::SIGN_MASK) != T::ZERO;

    if sign_set {
        // Build a mask of 1s covering the top `amount` bits, e.g. for amount=2:
        // start:      1111...1111   (!T::ZERO, all ones)
        // >> amount:  0011...1111   (logical shift right by amount)
        // !          :1100...0000   (invert -> top `amount` bits are 1, rest 0)
        let fill_mask = !((!T::ZERO) >> amount);
        shifted | fill_mask
    } else {
        shifted
    }
}

impl_alu! {
    add(a: A::Word, b: A::Word) ->
        value: a.overflowing_add(b),
        overflow: ((a ^ value) & (b ^ value) & A::Word::SIGN_MASK) != A::Word::ZERO
    sub(a: A::Word, b: A::Word) ->
        value: a.overflowing_sub(b),
        overflow: ((a ^ b) & (a ^ value) & A::Word::SIGN_MASK) != A::Word::ZERO

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
        value: (a << amount, shl_carry(a, amount)) ,
        overflow: {
            let old_sign = (a & A::Word::SIGN_MASK) != A::Word::ZERO;
            let new_sign = (value & A::Word::SIGN_MASK) != A::Word::ZERO;

            let overflow = amount == 1 && old_sign != new_sign;
            overflow
        }
    shr(a: A::Word, amount: u32) ->
        value: (a >> amount, shr_carry(a, amount)) ,
        overflow: amount == 1 && (a & A::Word::SIGN_MASK) != A::Word::ZERO
    sar(a: A::Word, amount: u32) ->
        value: (sar_value(a, amount), sar_carry(a, amount)),
        overflow: false // "Arithmentic shifting right" does not overflow

    not(a: A::Word) ->
        value: (!a, false),
        overflow: false
}

impl<A: Architecture> ArithmeticLogicUnit<A> {}
