use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::Path;

macro_rules! imports {
    ($file: expr, $($x: ident),*) => {
        write!($file, "use libc::{{")?;
        $(
			write!($file, "{},", stringify!($x))?;
		)*
		write!($file, "}};\n\n")?;
    };
}

macro_rules! _extern {
    ($file: expr, $block: block) => {
        write!($file, "extern \"C\" {{\n")?;
        $block;
        write!($file, "}}")?;
    };
}

fn main() -> Result<()> {
    let path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("bindings.rs");
    let mut file = BufWriter::new(File::create(&path)?);

    imports!(&mut file, c_float, c_double, uintptr_t);

    _extern!(&mut file, {
        macro_rules! comment {
            ($file: expr, $comment: expr) => {
                writeln!($file, concat!("\n\t/* ", $comment, " */"))?;
            };
        }

        macro_rules! def {
            ($file: expr, $str: tt $(, $arg: tt)*) => {
                writeln!($file, concat!("\tpub fn ", $str, ";"), $($arg)*)?;
            };
        }

        def!(&mut file, "BinaryenModuleCreate() -> usize");
        def!(&mut file, "BinaryenModuleDispose(module: usize)");
    });

    Ok(())
}
