use crate::{lexer::Token, operators};
use line_col::LineColLookup;
use logos::{Lexer, Logos};
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError(String);

impl ParseError {
    pub fn format_line(pos: (usize, usize), msg: &dyn AsRef<str>) -> ParseError {
        ParseError(format!("{}:{}: {}", pos.0, pos.1, msg.as_ref()))
    }

    pub fn new<E, G>(pos: (usize, usize), e: E, g: G) -> ParseError
    where
        E: fmt::Debug,
        G: fmt::Debug,
    {
        ParseError(format!(
            "{}:{}: expected {:?}, got {:?}",
            pos.0, pos.1, e, g
        ))
    }
}

macro_rules! error {
    ($pos: expr, $e: expr, $g: expr) => {
        Err(ParseError::new($pos, $e, $g))
    };
    ($pos: expr, $msg: expr) => {
        Err(ParseError::format_line($pos, $msg))
    };
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

pub enum Expr {
    Paren(Box<Expr>),
    Block(Vec<Expr>),
    Assignment(String, Box<Expr>),
    BinOp(Box<Expr>, String, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Ident(String),
    Integer(i64),
    Float(f64),
    Return(Box<Expr>),
}

pub enum Stat {
    Fn(String, Vec<Expr>, Expr),
    Data(u64, String),
}

pub type Root = Vec<Stat>;

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    peeked: Option<Option<Token>>,
    linecol: LineColLookup<'a>,
}

macro_rules! expect {
    ($compare: expr, $name: expr, $token_type: expr) => {
        if let Some(token) = $compare {
            if token == $token_type {
                token
            } else {
                panic!("expected {}, got {:?}", $name, token)
            }
        } else {
            panic!("expected {}, got <eof>", $name)
        }
    };
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a dyn AsRef<str>) -> Parser<'a> {
        Parser {
            lexer: Token::lexer(source.as_ref()),
            linecol: LineColLookup::new(source.as_ref()),
            peeked: None,
        }
    }

    fn pos(&self) -> (usize, usize) {
        self.linecol.get(self.lexer.span().end)
    }

    #[inline]
    fn peek(&mut self) -> Option<&Token> {
        let iter = &mut self.lexer;
        self.peeked.get_or_insert_with(|| iter.next()).as_ref()
    }

    #[inline]
    fn next(&mut self) -> Option<Token> {
        match self.peeked.take() {
            Some(v) => v,
            None => self.lexer.next(),
        }
    }

    #[inline]
    fn skip(&mut self) {
        self.next().unwrap();
    }

    fn prefixexpr(&mut self) -> Result<Expr> {
        match self.next() {
            Some(Token::OpenParen) => {
                let expr = Expr::Paren(Box::new(self.expr()?));
                expect!(self.next(), "')'", Token::CloseParen);
                Ok(expr)
            }
            Some(Token::Ident(s)) => Ok(Expr::Ident(s)),
            Some(Token::Float(f)) => Ok(Expr::Float(f)),
            Some(Token::Integer(i)) => Ok(Expr::Integer(i)),
            Some(t) => error!(self.pos(), "parentheses or literal in prefixexpr", t),
            None => error!(self.pos(), "prefixexpr", "<eof>"),
        }
    }

    fn primaryexpr(&mut self) -> Result<Expr> {
        let mut base = match self.peek() {
            Some(Token::Ident(s)) => {
                // TODO: possibility to remove to_owned here
                let owned = s.to_owned();
                self.skip();

                match self.peek() {
                    Some(&Token::Equals) => {
                        self.skip();
                        Expr::Assignment(owned, Box::new(self.expr()?))
                    }
                    _ => Expr::Ident(owned),
                }
            }
            Some(&Token::Return) => {
                self.skip();
                Expr::Return(Box::new(self.expr()?))
            }
            Some(&Token::OpenBrace) => {
                self.skip();
                Expr::Block(self.block()?)
            }
            Some(..) => self.prefixexpr()?,
            None => return error!(self.pos(), "expression", "<eof>"),
        };

        // arglist
        while let Some(Token::OpenParen) = self.peek() {
            self.skip();
            let mut arglist = Vec::new();

            loop {
                match self.peek() {
                    Some(&Token::CloseParen) => {
                        self.skip();
                        break;
                    }
                    Some(..) => {
                        arglist.push(self.expr()?);

                        match self.peek() {
                            Some(&Token::CloseParen) => {
                                self.skip();
                                break;
                            }
                            Some(&Token::Comma) => {
                                self.skip();
                                continue;
                            }
                            Some(..) => {
                                return error!(self.pos(), "')' or ','", self.next().unwrap())
                            }
                            None => return error!(self.pos(), "')' or ','", "<eof>"),
                        }
                    }
                    None => {
                        return error!(
                            self.pos(),
                            &"incomplete argument list, closing parenthesis not found"
                        )
                    }
                }
            }

            base = Expr::Call(Box::new(base), arglist)
        }

        Ok(base)
    }

    fn subexpr(&mut self, mut lhs: Expr, min_prec: u8) -> Result<Expr> {
        let mut peek = self.peek();
        loop {
            if let Some(Token::Op(op)) = peek {
                let op_prec = operators::precedence(op)
                    .unwrap_or_else(|| panic!("foreign operator {} found", op));

                if op_prec >= min_prec {
                    // TODO: possibility to remove to_owned here
                    let owned_op = op.to_owned();
                    self.skip();
                    let mut rhs = self.primaryexpr()?;

                    peek = self.peek();
                    loop {
                        if let Some(Token::Op(next_op)) = peek {
                            let next_op_prec = operators::precedence(next_op)
                                .unwrap_or_else(|| panic!("foreign operator {} found", next_op));

                            if next_op_prec > op_prec {
                                rhs = self.subexpr(rhs, min_prec + 1)?;
                                peek = self.peek();
                                continue;
                            }
                        }
                        break;
                    }

                    lhs = Expr::BinOp(Box::new(lhs), owned_op, Box::new(rhs));
                    continue;
                }
            }
            break;
        }
        Ok(lhs)
    }

    fn expr(&mut self) -> Result<Expr> {
        let lhs = self.primaryexpr()?;
        self.subexpr(lhs, 0)
    }

    fn block(&mut self) -> Result<Vec<Expr>> {
        let mut block = Vec::new();
        while self.peek().is_some() {
            block.push(self.expr()?);
            match self.peek() {
                Some(&Token::Semicolon) => {
                    self.skip();
                }
                _ => break,
            }
        }

        // expect close brace
        match self.next() {
            Some(Token::CloseBrace) => Ok(block),

            // TODO: add back semicolon error message
            Some(t) => error!(self.pos(), "'}'", t),
            None => error!(self.pos(), "'}'", "<eof>"),
        }
    }

    pub fn parse(&mut self) -> Result<Root> {
        let mut program = Vec::new();
        while let Some(tok) = self.peek() {
            match tok {
                &Token::Fn => {
                    self.skip();
                    match self.next() {
                        Some(Token::Ident(name)) => {
                            expect!(self.next(), "'('", Token::OpenParen);
                            expect!(self.next(), "')'", Token::CloseParen);
                            program.push(Stat::Fn(name, Vec::new(), self.expr()?));
                        }
                        Some(t) => return error!(self.pos(), "function name", t),
                        None => return error!(self.pos(), "function name", "<eof>"),
                    }
                }
                &Token::Data => {
                    self.skip();
                    expect!(self.next(), "'['", Token::OpenBracket);
                    if let Some(Token::Integer(pos)) = self.next() {
                        assert!(pos >= 0, "`data` position may not be negative");
                        expect!(self.next(), "']'", Token::CloseBracket);
                        expect!(self.next(), "'='", Token::Equals);
                        if let Some(Token::String(data)) = self.next() {
                            program.push(Stat::Data(pos as u64, data));
                        }
                    } else {
                        return error!(self.pos(), &"expected integer, e.g. data[<int>]");
                    }
                }
                &Token::Semicolon => self.skip(),
                _ => return error!(self.pos(), "data, fn or ';'", self.next().unwrap()),
            }
        }

        match self.next() {
            Some(t) => error!(self.pos(), "<eof>", t),
            None => Ok(program),
        }
    }
}
