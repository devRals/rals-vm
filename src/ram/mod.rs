use crate::cpu::isa::arch::Architecture;

/// RAM (Random Access Memory) is the place where all the temporary values have been stored and used
/// during the program execution
#[derive(Default)]
pub struct RandomAccessMemory<A: Architecture> {
    pub data: A::Memory,
}
