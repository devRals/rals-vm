use std::{
    env, fs,
    io::{self, ErrorKind},
    path::PathBuf,
};

use rals_assembler::Assembler;
use rals_vm_isa::arch::Arch32;

type Arch = Arch32;

fn new_error_result(msg: &str, kind: ErrorKind) -> anyhow::Result<()> {
    Err(anyhow::Error::new(io::Error::new(kind, msg.to_string())))
}

fn main() -> anyhow::Result<()> {
    let mut args = env::args();
    let path_to_source = {
        args.next();
        match args.next() {
            None => {
                return new_error_result("Please specify a source path", ErrorKind::NotFound);
            }
            Some(p) => PathBuf::from(p),
        }
    };
    let source = fs::read_to_string(path_to_source)?;
    let program = Assembler::<Arch>::parse(&source).map_err(anyhow::Error::new)?;
    let header = Assembler::<Arch>::resolve_header(&program);

    let header_values = match header {
        Some(header_result) => header_result?.directives,
        None => vec![],
    };
    let mut assembler = Assembler::<Arch>::new();
    let resolved = assembler.pass1(program)?;

    Ok(())
}
