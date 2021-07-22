use std::{
    fs::File,
    io::{self, Read},
};

use eretria::{lexer::Token, parse_string};
use logos::Logos;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn input_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("INPUT")
        .help("Sets the input file to use")
        .required(true)
}

fn file_to_string(input: impl AsRef<str>) -> io::Result<String> {
    let mut file = File::open(input.as_ref())?;
    let mut buf = String::with_capacity(file.metadata()?.len() as usize);
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() -> io::Result<()> {
    let matches = App::new("Eretria")
        .version("0.0.1")
        .author("romdotdog")
        .about("Generate 1:1 WebAssembly using a simple syntax")
        .subcommand(
            SubCommand::with_name("lex")
                .about("Dumps all tokens")
                .arg(input_arg()),
        )
        .subcommand(
            SubCommand::with_name("parse")
                .about("Dumps the AST")
                .arg(input_arg()),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Builds the program")
                .arg(input_arg()),
        )
        .get_matches();

    match matches.subcommand() {
        ("lex", Some(matches)) => {
            let input = matches.value_of("INPUT").expect("expected an input file");
            let buf = file_to_string(input)?;
            for tok in Token::lexer(&buf) {
                print!("{:?}  ", tok);
            }
        }
        ("parse", Some(matches)) => {
            let input = matches.value_of("INPUT").expect("expected an input file");
            let buf = file_to_string(input)?;
            print!("{:#?}", parse_string(buf).unwrap());
        }
        ("build", Some(matches)) => {
            let input = matches.value_of("INPUT").expect("expected an input file");
            let buf = file_to_string(input)?;
            parse_string(buf).unwrap();
        }
        _ => eprintln!("expected valid subcommand"),
    }
    Ok(())
}
