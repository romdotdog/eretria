use logos::{Lexer, Logos};

fn char_to_radix(c: char) -> Option<u8> {
    match c {
        'b' => Some(2),
        't' => Some(3),
        'q' => Some(4),
        'p' => Some(5),
        'h' => Some(6),
        's' => Some(7),
        'o' => Some(8),
        'e' => Some(9),
        'd' => Some(10),
        'l' => Some(11),
        'z' => Some(12),
        'x' => Some(16),
        _ => None,
    }
}

fn parse_float(lex: &mut Lexer<Token>) -> Option<f64> {
    let slice = lex.slice();
    let f: f64 = slice.parse().ok()?;
    Some(f)
}

fn parse_radix_number(lex: &mut Lexer<Token>) -> Option<u64> {
    let slice = lex.slice();
    let radix_char = slice.bytes().nth(1)? as char;
    let radix = char_to_radix(radix_char);
    assert!(
        radix.is_some(),
        "{} is not a valid radix prefix in number {}",
        radix_char,
        slice
    );
    let n: u64 = u64::from_str_radix(&slice[2..], radix.unwrap() as u32).ok()?;
    Some(n)
}

fn parse_number(lex: &mut Lexer<Token>) -> Option<u64> {
    let slice = lex.slice();
    let n: u64 = slice.parse().ok()?;
    Some(n)
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("global")]
    Global,

    #[token("export")]
    Export,

    #[token("fn")]
    Fn,

    #[token("=")]
    Equals,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token(";")]
    Semicolon,

    #[regex(r"\-?\d*\.\d+", parse_float, priority = 3)]
    Float(f64),

    #[regex(r"\-?0\D+", parse_radix_number, priority = 2)]
    #[regex(r"\-?\d+", parse_number, priority = 2)]
    Integer(u64),

    #[regex(r"\S+")]
    Ident,

    #[error]
    #[regex(r"\s+", logos::skip)]
    Error,
}
