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
    type Memory: MemoryStorage;

    const INSTRUCTION_SIZE: usize;
    /// MEMORY_SIZE basically determines how much the loaded program can be. Know that this size is
    /// limitted what [`Architecture::Word`] you choose because of jump ranges. For example you decided
    /// using u8 for [`Architecture::Word`] then your jump reach can be up to 255 which is [`u8::MAX`].
    /// Therefor if you exceed this limit your basically would try to acccess to memory address that
    /// doesn't exist
    const MEMORY_SIZE: usize;

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

pub trait MemoryStorage: AsBytes {}
pub trait InstructionStorage: AsBytes {}

impl<const N: usize> MemoryStorage for [u8; N] {}
impl MemoryStorage for Box<[u8]> {}
impl<const N: usize> InstructionStorage for [u8; N] {}

/// Creates a new struct that implements the [`Architecture`] trait
/// it implements all constants and functions automatically based on what Word you use
///
/// These constants are auto defined by the macro
/// MEMORY_SIZE: Word::MAX [`Architecture::MEMORY_SIZE`]
/// INSTRUCTION_SIZE: Word::BYTES + 4 ->
///     [opcode:1, reg:1, deref: [base: 1, index: 1, displacement: imm]]  
///     whats up there is LOAD ins example which takes the largest space in a instruction buffer
/// Instruction: [u8; Self::INSTRUCTION_SIZE] [`Architecture::MEMORY_SIZE`]
#[macro_export]
macro_rules! create_arch {
    ( $arch_name: ident {
        Word: $word: ty,
        MemorySize: $mem_size: expr
    }) => {
        #[derive(Default, Clone, Copy)]
        pub struct $arch_name;
        impl Architecture for $arch_name {
            type Word = $word;
            type Instruction = [u8; Self::INSTRUCTION_SIZE];
            type Memory = Box<[u8]>;

            const INSTRUCTION_SIZE: usize = Self::Word::BYTES + 4;
            const MEMORY_SIZE: usize = $mem_size;

            fn new_empty_memory() -> Self::Memory {
                vec![0u8; Self::MEMORY_SIZE].into_boxed_slice()
            }
        }
    };
}

create_arch!(Arch8 {
    Word: u8,
    MemorySize: 256 // full 8-bit range, cheap
});
create_arch!(Arch16 {
    Word: u16,
    MemorySize: 64 * 1024 // full 16-bit range
});
create_arch!(Arch32 {
    Word: u32,
    MemorySize: 16 * 1024 * 1024 // 16MB — plenty for a hobby VM
});
create_arch!(Arch64 {
    Word: u64,
    MemorySize: 32 * 1024 * 1024 // 32MB — same idea
});
