mod lexer;
mod operators;
mod parser;

use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use lexer::Token;
use logos::Logos;
use parser::Parser;

pub fn parse_string<'source>(s: &dyn AsRef<str>) {
    let lexer = Token::lexer(s.as_ref());
    let mut parser = Parser::new(lexer);
    parser.parse();
}

pub fn parse_file<T>(input: T) -> io::Result<()>
where
    T: AsRef<Path>,
{
    let mut file = File::open(input)?;
    let mut buf = String::with_capacity(file.metadata()?.len() as usize);
    file.read_to_string(&mut buf)?;
    Ok(parse_string(&buf))
}
