use std::{iter::Peekable, usize};

use crate::{lexer::Token, operators};
use line_col::LineColLookup;
use logos::{Lexer, Logos};
use std::fmt;

#[derive(Debug, Clone)]
struct ParseError<E, G>((usize, usize), E, G);

impl<E, G> fmt::Display for ParseError<E, G>
where
    E: fmt::Display,
    G: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}: expected {}, got {}",
            self.0 .0, self.0 .1, self.1, self.2
        )
    }
}

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

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a, Token>>,
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
            lexer: Token::lexer(source.as_ref()).peekable(),
            linecol: LineColLookup::new(source.as_ref()),
        }
    }

    #[inline]
    fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }

    #[inline]
    fn next(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    #[inline]
    fn skip(&mut self) {
        self.lexer.next().unwrap();
    }

    fn prefixexpr(&mut self) -> Expr {
        match self.next() {
            Some(Token::OpenParen) => {
                let expr = Expr::Paren(Box::new(self.expr()));
                expect!(self.next(), "')'", Token::CloseParen);
                expr
            }
            Some(Token::Ident(s)) => (Expr::Ident(s)),
            Some(Token::Float(f)) => (Expr::Float(f)),
            Some(Token::Integer(i)) => (Expr::Integer(i)),
            Some(t) => panic!("expected parentheses or literal in prefixexpr, got {:?}", t),
            None => panic!("expected prefixexpr, got <eof>"),
        }
    }

    fn primaryexpr(&mut self) -> Expr {
        let mut base = match self.peek() {
            Some(Token::Ident(s)) => {
                // TODO: possibility to remove to_owned here
                let owned = s.to_owned();
                self.skip();

                match self.peek() {
                    Some(&Token::Equals) => {
                        self.skip();
                        Expr::Assignment(owned, Box::new(self.expr()))
                    }
                    _ => Expr::Ident(owned),
                }
            }
            Some(&Token::Return) => {
                self.skip();
                Expr::Return(Box::new(self.expr()))
            }
            Some(&Token::OpenBrace) => {
                self.skip();
                Expr::Block(self.block())
            }
            Some(..) => self.prefixexpr(),
            None => panic!("expected expression, found nothing"),
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
                        arglist.push(self.expr());
                        match self.peek() {
                            Some(&Token::CloseParen) => {
                                self.skip();
                                break;
                            }
                            Some(&Token::Comma) => {
                                self.skip();
                                continue;
                            }
                            Some(t) => panic!("expected ')' or ',', got {:?}", t),
                            None => panic!("expected ')' or ',', got <eof>"),
                        }
                    }
                    None => panic!("incomplete argument list, closing parenthesis not found"),
                }
            }

            base = Expr::Call(Box::new(base), arglist)
        }

        base
    }

    fn subexpr(&mut self, mut lhs: Expr, min_prec: u8) -> Expr {
        let mut peek = self.peek();
        loop {
            if let Some(Token::Op(op)) = peek {
                let op_prec = operators::precedence(op)
                    .unwrap_or_else(|| panic!("foreign operator {} found", op));

                if op_prec >= min_prec {
                    // TODO: possibility to remove to_owned here
                    let owned_op = op.to_owned();
                    self.skip();
                    let mut rhs = self.primaryexpr();

                    peek = self.peek();
                    loop {
                        if let Some(Token::Op(next_op)) = peek {
                            let next_op_prec = operators::precedence(next_op)
                                .unwrap_or_else(|| panic!("foreign operator {} found", next_op));

                            if next_op_prec > op_prec {
                                rhs = self.subexpr(rhs, min_prec + 1);
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
        return lhs;
    }

    fn expr(&mut self) -> Expr {
        let lhs = self.primaryexpr();
        self.subexpr(lhs, 0)
    }

    fn block(&mut self) -> Vec<Expr> {
        let mut block = Vec::new();
        while self.peek().is_some() {
            block.push(self.expr());
            match self.peek() {
                Some(&Token::Semicolon) => {
                    self.skip();
                }
                _ => break,
            }
        }

        // expect close brace
        match self.next() {
            Some(Token::CloseBrace) => {}
            Some(t) => panic!(
                "expected '{}', found {:?} (are you missing a semicolon?)",
                '}', t
            ),
            None => panic!("expected '{}', found <eof>", '}'),
        };

        block
    }

    pub fn parse(&mut self) -> () {
        let mut program = Vec::new();
        while let Some(tok) = self.peek() {
            match tok {
                &Token::Fn => {
                    self.skip();
                    match self.next() {
                        Some(Token::Ident(name)) => {
                            expect!(self.next(), "'('", Token::OpenParen);
                            expect!(self.next(), "')'", Token::CloseParen);
                            program.push(Stat::Fn(name, Vec::new(), self.expr()));
                        }
                        Some(t) => panic!("expected function name, got {:?}", t),
                        None => panic!("expected function name, got <eof>"),
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
                        panic!("expected integer, e.g. data[<int>]")
                    }
                }
                _ => {}
            }
        }

        match self.next() {
            Some(t) => panic!("expected <eof>, got {:?}", t),
            None => {}
        }
    }
}
