mod compiler;
pub mod lexer;
mod operators;
mod parser;

use parser::Parser;

pub fn parse_string(s: impl AsRef<str>) -> parser::Result<parser::Program> {
    let mut parser = Parser::new(&s);
    parser.parse()
}
