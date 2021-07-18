use eretria::parse_file;
use std::io;

extern crate clap;
use clap::{App, Arg};

fn main() -> io::Result<()> {
    let matches = App::new("Eretria")
        .version("0.0.1")
        .author("romdotdog")
        .about("Generate 1:1 WebAssembly using a simple syntax")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input = matches.value_of("INPUT").expect("expected an input file");
    parse_file(input)?;
    Ok(())
}
