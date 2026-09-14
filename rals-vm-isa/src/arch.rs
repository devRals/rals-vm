use super::value::ImmediateValue;

/// Architecture defines how virtual system architecture should work with the decided components.
/// It decides how big should virtual cpu instructions will be, how big memory should be. what kind
/// of values should virtual cpu registers hold and so on.
pub trait Architecture: Sized {
    /// Word determines what type of value your Architecture should use in registers, immediate
    /// values etc.
    type Word: ImmediateValue;
    /// Instruction buffer type when encoding the instruction
    type Instruction: InstructionStorage;
    /// Memory type determines what type of memory should this architecture use.
    /// Memory is basically the where program is loaded an can manage it.
    type Memory: MemoryStorage<Self>;

    /// How many instructions in bytes a program can hold in itself
    const PROGRAM_SIZE: usize;

    /// Tells how many bytes the stack can hold
    const STACK_SIZE: usize;

    /// Tells how big the heap is in bytes.
    const HEAP_SIZE: usize;

    /// MEMORY_SIZE basically determines how much the loaded program can be. Know that this size is
    /// limitted what [`Architecture::Word`] you choose because of jump ranges. For example you decided
    /// using u8 for [`Architecture::Word`] then your jump reach can be up to 255 which is [`u8::MAX`].
    /// Therefor if you exceed this limit your basically would try to acccess to memory address that
    /// doesn't exist
    ///
    /// Auto defined by the sum of other constants beside [`Self::INSTRUCTION_SIZE`]
    const MEMORY_SIZE: usize = Self::PROGRAM_SIZE + Self::HEAP_SIZE + Self::STACK_SIZE;

    /// Instruction size defines how many bytes can take place in a instruction buffer while
    /// decoding and encoding. It's auto defined as it can take an ImmediateValue's byte count and 4
    /// extra bytes for opcode and operands
    const INSTRUCTION_SIZE: usize = Self::Word::BYTES + 4;

    fn new_empty_memory() -> Self::Memory;
}

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
    fn as_bytes_mut(&mut self) -> &mut [u8];
}

impl AsBytes for Box<[u8]> {
    fn as_bytes(&self) -> &[u8] {
        self
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl<const N: usize> AsBytes for [u8; N] {
    fn as_bytes(&self) -> &[u8] {
        self
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        self
    }
}

/// Used for dividing the memory into regions.
/// Usually this is automatically done by the OS (operating system) you're using
/// However this isn't the case in our option
pub trait MemoryStorage<A: Architecture>: AsBytes {
    fn get_program(&self) -> &[u8];
    fn get_program_mut(&mut self) -> &mut [u8];

    fn get_stack(&self) -> &[u8];
    fn get_stack_mut(&mut self) -> &mut [u8];

    fn get_heap(&self) -> &[u8];
    fn get_heap_mut(&mut self) -> &mut [u8];
}

impl<A: Architecture> MemoryStorage<A> for Box<[u8]> {
    fn get_program(&self) -> &[u8] {
        let start = 0;
        let end = A::PROGRAM_SIZE;
        &self[start..end]
    }
    fn get_program_mut(&mut self) -> &mut [u8] {
        let start = 0;
        let end = A::PROGRAM_SIZE;
        &mut self[start..end]
    }

    fn get_heap(&self) -> &[u8] {
        let start = A::PROGRAM_SIZE;
        let end = start + A::HEAP_SIZE;
        &self[start..end]
    }
    fn get_heap_mut(&mut self) -> &mut [u8] {
        let start = A::PROGRAM_SIZE;
        let end = start + A::HEAP_SIZE;
        &mut self[start..end]
    }

    fn get_stack(&self) -> &[u8] {
        let start = A::PROGRAM_SIZE + A::HEAP_SIZE;
        let end = start + A::STACK_SIZE;

        &self[start..end]
    }
    fn get_stack_mut(&mut self) -> &mut [u8] {
        let start = A::PROGRAM_SIZE + A::HEAP_SIZE;
        let end = start + A::STACK_SIZE;

        &mut self[start..end]
    }
}

pub trait InstructionStorage: AsBytes {}
impl<const N: usize> InstructionStorage for [u8; N] {}

#[macro_export]
macro_rules! create_arch {
    ( $arch_name: ident {
        Word: $word: ty,
        ProgramSize: $program_size: expr,
        HeapSize: $heap_size: expr,
        StackSize: $stack_size: expr $(,)?
    }) => {
        #[derive(Default, Clone, Copy)]
        pub struct $arch_name;
        impl Architecture for $arch_name {
            type Word = $word;
            type Instruction = [u8; Self::INSTRUCTION_SIZE];
            /// Regions: [program, heap, call_stack, stack]
            type Memory = Box<[u8]>;

            const PROGRAM_SIZE: usize = $program_size * Self::INSTRUCTION_SIZE;
            const HEAP_SIZE: usize = $heap_size;
            const STACK_SIZE: usize = $stack_size * Self::Word::BYTES;

            fn new_empty_memory() -> Self::Memory {
                vec![0u8; Self::MEMORY_SIZE].into_boxed_slice()
            }
        }
    };
}

create_arch!(Arch8 {
    Word: u8,
    ProgramSize: 128,
    HeapSize: 256,
    StackSize: 32,
});
create_arch!(Arch16 {
    Word: u16,
    ProgramSize: 256,
    HeapSize: 256,
    StackSize: 64,
});
create_arch!(Arch32 {
    Word: u32,
    ProgramSize: 256,
    HeapSize: 512,
    StackSize: 128,
});
create_arch!(Arch64 {
    Word: u64,
    ProgramSize: 512,
    HeapSize: 1024,
    StackSize: 256,
});
