use std::iter::Peekable;

use crate::lexer::Token;
use logos::Lexer;

pub enum Expr {
    Paren(Box<Expr>),
    Assignment(Token, Box<Expr>),
    Literal(Token),
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a, Token>>,
}

macro_rules! expect {
    ($compare: expr, $name: expr, $token_type: expr) => {
        if let Some(token) = $compare {
            if token != $token_type {
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
                expect!(self.lexer.next(), "CloseParen", Token::CloseParen);
                Box::new(expr)
            }
            Token::Ident | Token::F64 | Token::F32 | Token::I64 | Token::I32 => {
                Box::new(Expr::Literal(t))
            }
            _ => panic!("expected parentheses or literal in prefixexpr"),
        }
    }

    fn expr(&mut self) -> Box<Expr> {
        if let Some(p) = self.lexer.peek() {
            if p == &Token::Ident {
                let pn = self.lexer.next().unwrap();
                if let Some(p1) = self.lexer.peek() {
                    if p1 == &Token::Equals {
                        self.lexer.next().unwrap();
                        return Box::new(Expr::Assignment(pn, self.expr()));
                    }
                }
                return Box::new(Expr::Literal(pn));
            }
        }
        return self.prefixexpr();
    }

    pub fn block(&mut self) -> Vec<Box<Expr>> {
        let mut block = Vec::new();
        loop {
            block.push(self.expr());
            if let Some(tok) = self.lexer.peek() {
                if tok != &Token::Semicolon {
                    break;
                }
            } else {
                break;
            }
        }
        block
    }
}
