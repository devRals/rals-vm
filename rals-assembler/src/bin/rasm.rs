use std::{
    env,
    fs::{self, File},
    io::{self, ErrorKind, Write},
    path::PathBuf,
};

use rals_assembler::{
    Assembler,
    ast::{AstItem, AstProgram},
};
use rals_vm_isa::arch::{Arch8, Arch16, Arch32, Arch64, Architecture};

const RALS_VM_EXTENTION: &str = "rvm";
const RALS_VM_ASSEMBLY_EXTENTION: &str = "rasm";

type DefaultArch = Arch32;

fn main() -> anyhow::Result<()> {
    let mut args = env::args();
    args.next();
    let path_as_string = args.next().ok_or(io::Error::new(
        ErrorKind::InvalidInput,
        "Please specify a path to source file",
    ))?;
    let path_to_source = PathBuf::try_from(path_as_string)?;
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
    let program = rals_assembler::parse(&source).map_err(anyhow::Error::new)?;

    if let Some(header) = program
        .sections
        .iter()
        .find(|s| s.name.to_lowercase() == "header".to_string())
        && let Some(arch) = header
            .items
            .iter()
            .find(|d| matches!(d, AstItem::Directive { key, value } if key == &"arch".to_string()))
    {
        match arch {
            AstItem::Directive { key: _, value } => match *value {
                8 => run::<Arch8>(program, path_to_source),
                16 => run::<Arch16>(program, path_to_source),
                32 => run::<Arch32>(program, path_to_source),
                64 => run::<Arch64>(program, path_to_source),
                _ => run::<DefaultArch>(program, path_to_source),
            },
            _ => unreachable!(),
        }
    } else {
        run::<DefaultArch>(program, path_to_source)
    }
}

fn run<A: Architecture>(program: AstProgram, mut path_to_source: PathBuf) -> anyhow::Result<()> {
    let mut assembler = Assembler::<A>::new();
    let resolved = assembler.pass1(program)?;
    let bytecode = assembler.pass2(resolved);

    path_to_source.set_extension(RALS_VM_EXTENTION);
    let mut bytecode_file = File::create(path_to_source)?;
    bytecode_file.write_all(&bytecode)?;

    println!["{bytecode:?}"];

    Ok(())
}
