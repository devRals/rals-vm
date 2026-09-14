use rals_vm_isa::arch::Architecture;
use rals_vm_isa::value::ImmediateValue;

/// PC (Program Counter not Personal Computer) is a physical&hardware CPU Component that
/// tells us which instruction were currently executing. Every time the
/// instruction loop (fetch -> decode -> execute -> repeat) ends
/// PC increases which means we skipped to the next instruction in a program
#[derive(Default)]
pub struct ProgramCounter<A: Architecture> {
    pub stopped: bool,
    counter: A::Word,
}

impl<A: Architecture> ProgramCounter<A> {
    pub const fn new() -> Self {
        ProgramCounter {
            counter: A::Word::ZERO,
            stopped: false,
        }
    }

    pub fn advance(&mut self) {
        if !self.stopped {
            self.counter = self.next()
        }
    }

    /// Returns next possible instruction
    pub fn next(&self) -> A::Word {
        // Wrap around
        self.counter.wrapping_add(A::Word::ONE)
    }

    /// JMP (Jump) goes to the given instruction address and continues from at that point
    pub fn jmp(&mut self, addr: A::Word) {
        if !self.stopped {
            self.counter = addr;
        }
    }

    /// JMR (Jump Relative) skips `amount` amount of instructions and continues from at that point
    pub fn jmr(&mut self, amount: A::Word) {
        if !self.stopped {
            let next = self.counter.wrapping_add(amount);
            self.counter = next;
        }
    }

    /// HLT (Halt) stops whole instruction fetch/decode/execute flow
    pub fn hlt(&mut self) {
        self.stopped = true;
    }

    pub const fn get_word(&self) -> A::Word {
        self.counter
    }

    pub fn get(&self) -> usize {
        self.counter.as_usize() * A::INSTRUCTION_SIZE
    }

    pub fn reset(&mut self) {
        self.counter = A::Word::ZERO
    }
}
