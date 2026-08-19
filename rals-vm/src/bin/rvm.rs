use std::{fs, path::PathBuf};

use clap::{Parser, ValueEnum};
use rals_vm::VirtualMachine;
use rals_vm_isa::arch::{Arch8, Arch16, Arch32, Arch64, Architecture};

/// RVM (Rals Virtual Machine) is a virtual cpu executor from RVMF (Rals VM Formatted) binary files
#[derive(Parser)]
pub struct Cli {
    /// Which architecture should rvm use. Default = 32
    #[arg(short, long)]
    arch: Option<Arch>,

    #[command()]
    rvm_file: PathBuf,
}

#[derive(ValueEnum, Clone)]
#[value()]
pub enum Arch {
    Arch8 = 8,
    Arch16 = 16,
    Arch32 = 32,
    Arch64 = 64,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let mut bytecode = fs::read(args.rvm_file)?;
    resolve_header(&mut bytecode);

    match args.arch.unwrap_or(Arch::Arch32) {
        Arch::Arch8 => run::<Arch8>(&bytecode),
        Arch::Arch16 => run::<Arch16>(&bytecode),
        Arch::Arch32 => run::<Arch32>(&bytecode),
        Arch::Arch64 => run::<Arch64>(&bytecode),
    }
    Ok(())
}

fn resolve_header(_bytecode: &mut Vec<u8>) {}

fn run<A: Architecture>(bytecode: &[u8]) {
    let mut vm = VirtualMachine::<A>::new();
    vm.set_mem(bytecode);
    vm.run();
}
