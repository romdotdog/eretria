use logos::Logos;

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

    #[regex(r"[0-9]*\.[0-9]+!", priority = 5)]
    F64,

    #[regex(r"[0-9]*\.[0-9]+", priority = 4)]
    F32,

    #[regex(r"[0-9]+!", priority = 3)]
    I64,

    #[regex(r"[0-9]+", priority = 2)]
    I32,

    #[regex(r"\S+")]
    Ident,

    #[error]
    #[regex(r"\s+", logos::skip)]
    Error,
}
