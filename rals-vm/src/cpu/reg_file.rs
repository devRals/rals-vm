use rals_vm_isa::{arch::Architecture, registers::Register, value::ImmediateValue};

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
            if self.counter == A::Word::MAX {
                panic!(
                    "reached the maximum program counter value. Consider using a bigger architecture word"
                )
            }
            self.counter = self.counter + A::Word::ONE;
        }
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

    pub fn get(&self) -> usize {
        self.counter.as_usize() * A::INSTRUCTION_SIZE
    }
}

/// RegisterFile is the place where all the CPU registers take place
pub struct RegisterFile<A: Architecture> {
    /// We have 16 register and everyone of the are the same size.
    pub general: [A::Word; 16],
    pub pc: ProgramCounter<A>,
}

impl<A: Architecture> RegisterFile<A> {
    /// Zero registers in CPU's cant be written and their values are always zero
    const ZERO_REGISTER: Register = Register::R0;

    pub const fn new() -> Self {
        RegisterFile {
            pc: ProgramCounter::new(),
            general: [A::Word::ZERO; 16],
        }
    }

    pub fn reset(&mut self) {
        self.pc.counter = A::Word::ZERO;
        self.general.fill(A::Word::ZERO);
    }

    /// LDI (Load Immediate) changes the `dst` register value to given value
    pub const fn ldi(&mut self, dst: Register, src: A::Word) {
        self.general[dst as usize] = src;
    }

    /// MOV (move) changes `dst` register value to `src` register value
    pub const fn mov(&mut self, dst: Register, src: Register) {
        self.general[dst as usize] = self.general[src as usize]
    }

    /// INC (Increase) increases the given register value
    pub fn inc(&mut self, dst: Register) {
        self.general[dst as usize] = self.general[dst as usize].wrapping_add(A::Word::ONE);
    }

    /// DEC (Decrease) decreases the given register value
    pub fn dec(&mut self, dst: Register) {
        self.general[dst as usize] = self.general[dst as usize].wrapping_sub(A::Word::MAX);
    }

    pub fn set_reg(&mut self, reg: Register, value: A::Word) {
        // zero registers physically cannot be changed
        if reg == Self::ZERO_REGISTER {
            return;
        }
        if let Register::UnknownRegister = reg {
            panic!("recieved an unknown register. please update your code")
        }
        self.general[reg as usize] = value
    }
}
