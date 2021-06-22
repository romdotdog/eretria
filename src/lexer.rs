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

fn parse_radix_number(lex: &mut Lexer<Token>) -> Option<i64> {
    let slice = lex.slice();
    let radix_char = slice.bytes().nth(1)? as char;
    let radix = char_to_radix(radix_char);
    assert!(
        radix.is_some(),
        "{} is not a valid radix prefix in number {}",
        radix_char,
        slice
    );
    let n: i64 = i64::from_str_radix(&slice[2..], radix.unwrap() as u32).ok()?;
    Some(n)
}

fn parse_number(lex: &mut Lexer<Token>) -> Option<i64> {
    let slice = lex.slice();
    let n: i64 = slice.parse().ok()?;
    Some(n)
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("global")]
    Global,

    #[token("export")]
    Export,

    #[token("return")]
    Return,

    #[token("fn")]
    Fn,

    #[token("=", priority = 3)]
    Equals,

    #[token("(", priority = 3)]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{", priority = 3)]
    OpenBrace,

    #[token("}", priority = 3)]
    CloseBrace,

    #[token(";", priority = 3)]
    Semicolon,

    #[regex(r"\-?\d*\.\d+", parse_float, priority = 4)]
    Float(f64),

    #[regex(r"\-?0\D\S+", parse_radix_number, priority = 3)]
    #[regex(r"\-?\d+", parse_number, priority = 3)]
    Integer(i64),

    #[regex(r"[^\r\n\t\f\v ();]+", |lex| lex.slice().to_owned(), priority = 2)]
    Ident(String),

    #[error]
    #[regex(r"\s+", logos::skip)]
    Error,
}
