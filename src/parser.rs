use std::iter::Peekable;

use crate::{lexer::Token, operators};
use logos::Lexer;

pub enum Expr {
    Paren(Box<Expr>),
    Block(Vec<Box<Expr>>),
    Assignment(String, Box<Expr>),
    BinOp(Box<Expr>, String, Box<Expr>),
    Ident(String),
    Integer(i64),
    Float(f64),
    Return,
}

pub enum Stat {
    Fn(String, Vec<Box<Expr>>, Box<Expr>),
    Data(u64, String),
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a, Token>>,
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

impl Parser<'_> {
    pub fn new(lexer: Lexer<Token>) -> Parser {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    fn prefixexpr(&mut self) -> Box<Expr> {
        let t = self.lexer.next().expect("expected prefixexpr, got <eof>");
        match t {
            Token::OpenParen => {
                let expr = Expr::Paren { 0: self.expr() };
                expect!(self.lexer.next(), "')'", Token::CloseParen);
                Box::new(expr)
            }
            Token::Ident(s) => Box::new(Expr::Ident(s)),
            Token::Float(f) => Box::new(Expr::Float(f)),
            Token::Integer(i) => Box::new(Expr::Integer(i)),
            _ => panic!("expected parentheses or literal in prefixexpr, got {:?}", t),
        }
    }

    fn primaryexpr(&mut self) -> Box<Expr> {
        if let Some(p) = self.lexer.peek() {
            match p {
                &Token::Ident(..) => {
                    if let Some(Token::Ident(s)) = self.lexer.next() {
                        if let Some(p1) = self.lexer.peek() {
                            if p1 == &Token::Equals {
                                self.lexer.next().unwrap();
                                return Box::new(Expr::Assignment(s, self.expr()));
                            }
                        }
                        return Box::new(Expr::Ident(s));
                    } else {
                        unreachable!()
                    }
                }
                &Token::Return => {
                    self.lexer.next().unwrap();
                    return Box::new(Expr::Return);
                }
                &Token::OpenBrace => {
                    self.lexer.next().unwrap();
                    return Box::new(Expr::Block(self.block()));
                }
                _ => return self.prefixexpr(),
            }
        }
        panic!("expected expression, found nothing")
    }

    fn subexpr(&mut self, mut lhs: Box<Expr>, min_prec: u8) -> Box<Expr> {
        let mut peek = self.lexer.peek();
        loop {
            if let Some(Token::Op(op)) = peek {
                let op_prec = operators::precedence(op)
                    .unwrap_or_else(|| panic!("foreign operator {} found", op));

                if op_prec >= min_prec {
                    // op is borrowed, owned_op is not
                    let owned_op = match self.lexer.next() {
                        Some(Token::Op(o)) => o,
                        _ => unreachable!(),
                    };

                    let mut rhs = self.primaryexpr();

                    peek = self.lexer.peek();
                    loop {
                        if let Some(Token::Op(next_op)) = peek {
                            let next_op_prec = operators::precedence(next_op)
                                .unwrap_or_else(|| panic!("foreign operator {} found", next_op));

                            if next_op_prec > op_prec {
                                rhs = self.subexpr(rhs, min_prec + 1);
                                peek = self.lexer.peek();
                                continue;
                            }
                        }
                        break;
                    }

                    lhs = Box::new(Expr::BinOp(lhs, owned_op, rhs));

                    continue;
                }
            }
            break;
        }
        return lhs;
    }

    fn expr(&mut self) -> Box<Expr> {
        let lhs = self.primaryexpr();
        self.subexpr(lhs, 0)
    }

    fn block(&mut self) -> Vec<Box<Expr>> {
        let mut block = Vec::new();
        while self.lexer.peek().is_some() {
            block.push(self.expr());
            if let Some(tok) = self.lexer.peek() {
                if tok == &Token::Semicolon {
                    self.lexer.next().unwrap();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        let next = self.lexer.next();
        assert!(next.is_some(), "expected '{}', found <eof>.", "}");

        assert_eq!(
            next.unwrap(),
            Token::CloseBrace,
            "block didn't end correctly. (are you missing a semicolon?)"
        );

        block
    }

    pub fn parse(&mut self) -> () {
        let mut program = Vec::new();
        while let Some(tok) = self.lexer.peek() {
            match tok {
                &Token::Fn => {
                    self.lexer.next().unwrap();
                    if let Some(Token::Ident(name)) = self.lexer.next() {
                        expect!(self.lexer.next(), "'('", Token::OpenParen);
                        expect!(self.lexer.next(), "')'", Token::CloseParen);
                        program.push(Stat::Fn(name, Vec::new(), self.expr()));
                    } else {
                        // TODO: make error message better
                        panic!("expected function name.");
                    }
                }
                &Token::Data => {
                    self.lexer.next().unwrap();
                    expect!(self.lexer.next(), "'['", Token::OpenBracket);
                    if let Some(Token::Integer(pos)) = self.lexer.next() {
                        assert!(pos > -1, "`data` position may not be negative");
                        expect!(self.lexer.next(), "']'", Token::CloseBracket);
                        expect!(self.lexer.next(), "'='", Token::Equals);
                        if let Some(Token::String(data)) = self.lexer.next() {
                            program.push(Stat::Data(pos as u64, data));
                        }
                    } else {
                        panic!("expected integer, e.g. data[<int>]")
                    }
                }
                _ => {}
            }
        }
    }
}
