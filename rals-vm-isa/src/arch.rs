use super::value::ImmediateValue;

/// Architecture defines how virtual system architecture should work with the decided components.
/// It decides how big should virtual cpu instructions will be, how big memory should be. what kind
/// of values should virtual cpu registers hold and so on.
pub trait Architecture: Sized {
    type Word: ImmediateValue;
    type Instruction: InstructionStorage;
    type Memory: MemoryStorage;

    const INSTRUCTION_SIZE: usize;
    const MEMORY_SIZE: usize;
}

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
    fn as_bytes_mut(&mut self) -> &mut [u8];
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
impl<const N: usize> InstructionStorage for [u8; N] {}

#[macro_export]
macro_rules! create_arch {
    ( $arch_name: ident {
        Word: $word: ty,
        InstructionSize: $instruction_size: expr,
        MemorySize: $memory_size: expr
    }) => {
        #[derive(Default, Clone, Copy)]
        pub struct $arch_name;
        impl Architecture for $arch_name {
            type Word = $word;
            type Instruction = [u8; $instruction_size];
            type Memory = [u8; $memory_size];

            const INSTRUCTION_SIZE: usize = $instruction_size;
            const MEMORY_SIZE: usize = $memory_size;
        }
    };
}

create_arch!(Arch8 {
    Word: u8,
    InstructionSize: 4,
    MemorySize: 512
});
create_arch!(Arch16 {
    Word: u16,
    InstructionSize: 8,
    MemorySize: 1024
});
create_arch!(Arch32 {
    Word: u32,
    InstructionSize: 8,
    MemorySize: 2048
});
create_arch!(Arch64 {
    Word: u64,
    InstructionSize: 16,
    MemorySize: 4096
});
