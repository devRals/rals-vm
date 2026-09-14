use rals_vm_isa::Decode;
use rals_vm_isa::arch::{Architecture, MemoryStorage};
use rals_vm_isa::value::ImmediateValue;

/// A stack pointer is a small register that stores the memory address of the last data element added to the stack or,
/// in some cases, the first available address in the stack. A stack is a specialized buffer that is
/// used by a program's functions to store data such as parameters, local variables and other function-related information.
///
/// The stack stores data from the top down, following a last in, first out (LIFO) data structure.
pub struct StackPointer<A: Architecture> {
    addr: A::Word,
}

impl<A: Architecture> StackPointer<A> {
    pub fn new() -> Self {
        StackPointer {
            addr: A::Word::try_from_usize(A::STACK_SIZE)
                .expect("rals-vm: Failed to construct stack pointer. Try a diffirent architecture"),
        }
    }

    pub const fn set(&mut self, value: A::Word) {
        self.addr = value;
    }

    pub const fn get(&self) -> A::Word {
        self.addr
    }

    pub fn push(&mut self, mem: &mut A::Memory, value: A::Word) {
        let stack = mem.get_stack_mut();
        let size = A::Word::try_from_usize(A::Word::BYTES).unwrap();

        let new_sp = self
            .addr
            .checked_sub(size)
            .expect("rals-vm: stack overflowed");

        let data = value.to_bytes();
        let (start, end) = (new_sp.as_usize(), self.addr.as_usize());
        stack[start..end].copy_from_slice(data.as_ref());

        self.addr = new_sp;
    }

    pub fn pop(&mut self, mem: &A::Memory) -> A::Word {
        let stack = mem.get_stack();
        let size = A::Word::try_from_usize(A::Word::BYTES).unwrap();

        let new_sp = self
            .addr
            .checked_add(size)
            .expect("rals-vm: stack underflowed");

        if new_sp.as_usize() > A::STACK_SIZE {
            panic!("rals-vm: stack underflowed");
        }

        let (start, end) = (self.addr.as_usize(), new_sp.as_usize());
        let value = Decode::decode(&stack[start..end]);

        self.addr = new_sp;

        value
    }
}
