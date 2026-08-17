use std::{
    env,
    fs::{self, File},
    io::{self, ErrorKind, Write},
    path::PathBuf,
};

use rals_assembler::Assembler;
use rals_vm_isa::arch::Arch32;

const RALS_VM_EXTENTION: &str = "rvm";
const RALS_VM_ASSEMBLY_EXTENTION: &str = "rasm";

type Arch = Arch32;

fn main() -> anyhow::Result<()> {
    let mut args = env::args();
    args.next();
    let path_as_string = args.next().ok_or(io::Error::new(
        ErrorKind::InvalidInput,
        "Please specify a path to source file",
    ))?;
    let mut path_to_source = PathBuf::try_from(path_as_string)?;
    if let Some(ext) = path_to_source.extension()
        && let Some(ext) = ext.to_str()
        && ext != RALS_VM_ASSEMBLY_EXTENTION
    {
        return Err(io::Error::new(
            ErrorKind::InvalidFilename,
            format!("`{ext}` is not a valid extension name. use `{RALS_VM_ASSEMBLY_EXTENTION}`"),
        )
        .into());
    }
    let source = fs::read_to_string(&path_to_source)?;
    let program = Assembler::<Arch>::parse(&source).map_err(anyhow::Error::new)?;

    let mut assembler = Assembler::<Arch>::new();
    let resolved = assembler.pass1(program)?;
    let bytecode = assembler.pass2(resolved)?;

    path_to_source.set_extension(RALS_VM_EXTENTION);
    let mut bytecode_file = File::create(path_to_source)?;
    bytecode_file.write_all(&bytecode)?;

    Ok(())
}
