mod lexer;
mod operators;
mod parser;

use lexer::Token;
use logos::Logos;
use parser::Parser;
use std::fs::File;
use std::io::Read;

extern crate clap;
use clap::{App, Arg};

fn main() {
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
    let mut file = File::open(input).expect("could not open file. (does it exist?)");
    let mut buf = String::with_capacity(
        file.metadata().expect("could not get file metadata.").len() as usize,
    );
    file.read_to_string(&mut buf)
        .expect("file contains invalid utf-8");

    let lexer = Token::lexer(buf.as_str());
    let mut parser = Parser::new(lexer);
    parser.parse();
}
