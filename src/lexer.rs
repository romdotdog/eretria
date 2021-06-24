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

// Questionable code..
fn parse_string(lex: &mut Lexer<Token>) -> Option<String> {
    let mut remainder = lex.remainder().chars().enumerate().peekable();
    let mut res = String::new();
    loop {
        match remainder.next() {
            Some((_, '\\')) => {
                // check the first character
                let (_, header_char) = remainder
                    .next()
                    .expect("expected escape character, got <eof>");

                if let Some(radix) = char_to_radix(header_char) {
                    //  ^ \x6e, \d71
                    // if there is no applicable digit after, then treat as normal escape
                    let (_, header_digit) = remainder
                        .peek()
                        .expect("expected integer escape, got <eof>");
                    if header_digit.is_digit(radix as u32) {
                        // go on parsing as normal
                        let mut num = String::from(remainder.next().unwrap().1);
                        while {
                            let peek = remainder.peek();
                            peek.is_some() && peek.unwrap().1.is_digit(radix as u32)
                        } {
                            // if the peeked character is a digit, then push it
                            num.push(remainder.next().unwrap().1);
                        }

                        let codepoint = u32::from_str_radix(&num, radix as u32);
                        assert!(codepoint.is_ok(), "max escape value is {}", u32::MAX);

                        let escaped = char::from_u32(codepoint.unwrap());
                        assert!(
                            escaped.is_some(),
                            "invalid codepoint, cannot add \\{}{} as part of escape",
                            header_char,
                            num
                        );

                        // push the codepoint
                        res.push(escaped.unwrap());
                    } else {
                        // treat header character as escaped
                        res.push(header_char);
                    }
                } else {
                    // is not an escape
                    res.push(header_char);
                }
            }
            Some((i, '"')) => {
                lex.bump(i + 1);
                break;
            }
            Some((_, c)) => res.push(c),
            None => panic!("incomplete string literal"),
        }
    }

    Some(res)
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("data")]
    Data,

    #[token("\"", parse_string, priority = 3)]
    String(String),

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

    #[token(")", priority = 3)]
    CloseParen,

    #[token("{", priority = 3)]
    OpenBrace,

    #[token("}", priority = 3)]
    CloseBrace,

    #[token("[", priority = 3)]
    OpenBracket,

    #[token("]", priority = 3)]
    CloseBracket,

    #[token(";", priority = 3)]
    Semicolon,

    #[regex(r"\^|\||&|==|!=|>=|<=|>>|<<|>|<|\+|-|\*\*|/|\*", |lex| lex.slice().to_owned(), priority = 4)]
    Op(String),

    #[regex(r"\-?\d*\.\d+", parse_float, priority = 4)]
    Float(f64),

    #[regex(r"\-?0[^\d\s]\S+", parse_radix_number, priority = 3)]
    #[regex(r"\-?\d+", parse_number, priority = 3)]
    Integer(i64),

    // TODO: figure out some way to become catch-all
    #[regex(r"[_A-Za-z][0-9_A-Za-z]+", |lex| lex.slice().to_owned(), priority = 2)]
    Ident(String),

    #[error]
    #[regex(r"\s+", logos::skip)]
    Error,
}
