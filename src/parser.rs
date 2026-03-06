use crate::ast::{Expr, Pauli};
use crate::scalar::Scalar;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedEnd,
    UnexpectedToken(char),
    UnknownSymbol(String),
    TrailingInput(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedEnd => write!(f, "unexpected end of input"),
            ParseError::UnexpectedToken(token) => write!(f, "unexpected token: {token}"),
            ParseError::UnknownSymbol(symbol) => write!(f, "unknown symbol: {symbol}"),
            ParseError::TrailingInput(rest) => write!(f, "unexpected trailing input: {rest}"),
        }
    }
}

impl Error for ParseError {}

pub fn parse_expr(input: &str) -> Result<Expr, ParseError> {
    let mut parser = Parser::new(input);
    let expr = parser.parse_addition()?;
    parser.skip_ws();
    if parser.is_eof() {
        Ok(expr)
    } else {
        Err(ParseError::TrailingInput(parser.remaining().to_string()))
    }
}

struct Parser<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, index: 0 }
    }

    fn parse_addition(&mut self) -> Result<Expr, ParseError> {
        let mut terms = vec![self.parse_multiplication()?];
        loop {
            self.skip_ws();
            if self.consume_char('+') {
                terms.push(self.parse_multiplication()?);
            } else {
                break;
            }
        }

        Ok(match terms.len() {
            1 => terms.pop().expect("single parsed term"),
            _ => Expr::Add(terms),
        })
    }

    fn parse_multiplication(&mut self) -> Result<Expr, ParseError> {
        let mut factors = vec![self.parse_atom()?];
        loop {
            self.skip_ws();
            if self.consume_char('*') {
                factors.push(self.parse_atom()?);
            } else {
                break;
            }
        }

        Ok(match factors.len() {
            1 => factors.pop().expect("single parsed factor"),
            _ => Expr::Mul(factors),
        })
    }

    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        self.skip_ws();
        let next = self.peek_char().ok_or(ParseError::UnexpectedEnd)?;
        match next {
            '(' => {
                self.index += 1;
                let expr = self.parse_addition()?;
                self.skip_ws();
                if self.consume_char(')') {
                    Ok(expr)
                } else {
                    Err(ParseError::UnexpectedEnd)
                }
            }
            '[' => self.parse_commutator(),
            '0'..='9' => self.parse_integer(),
            'I' | 'X' | 'Y' | 'Z' | 'i' => self.parse_symbol(),
            ch if ch.is_ascii_alphabetic() => Err(ParseError::UnknownSymbol(self.read_ident())),
            ch => Err(ParseError::UnexpectedToken(ch)),
        }
    }

    fn parse_commutator(&mut self) -> Result<Expr, ParseError> {
        self.expect_char('[')?;
        let lhs = self.parse_addition()?;
        self.skip_ws();
        self.expect_char(',')?;
        let rhs = self.parse_addition()?;
        self.skip_ws();
        self.expect_char(']')?;
        Ok(Expr::Commutator(Box::new(lhs), Box::new(rhs)))
    }

    fn parse_integer(&mut self) -> Result<Expr, ParseError> {
        let start = self.index;
        while matches!(self.peek_char(), Some(ch) if ch.is_ascii_digit()) {
            self.index += 1;
        }
        let value = self.input[start..self.index]
            .parse::<i32>()
            .map_err(|_| ParseError::UnexpectedToken(self.peek_char().unwrap_or('?')))?;
        Ok(Expr::Scalar(Scalar::from_int(value)))
    }

    fn parse_symbol(&mut self) -> Result<Expr, ParseError> {
        let ch = self.peek_char().ok_or(ParseError::UnexpectedEnd)?;
        self.index += 1;
        Ok(match ch {
            'I' => Expr::Sym(Pauli::I),
            'X' => Expr::Sym(Pauli::X),
            'Y' => Expr::Sym(Pauli::Y),
            'Z' => Expr::Sym(Pauli::Z),
            'i' => Expr::Scalar(Scalar::i()),
            _ => return Err(ParseError::UnknownSymbol(ch.to_string())),
        })
    }

    fn expect_char(&mut self, expected: char) -> Result<(), ParseError> {
        self.skip_ws();
        match self.peek_char() {
            Some(ch) if ch == expected => {
                self.index += 1;
                Ok(())
            }
            Some(ch) => Err(ParseError::UnexpectedToken(ch)),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    fn consume_char(&mut self, expected: char) -> bool {
        self.skip_ws();
        match self.peek_char() {
            Some(ch) if ch == expected => {
                self.index += 1;
                true
            }
            _ => false,
        }
    }

    fn read_ident(&mut self) -> String {
        let start = self.index;
        while matches!(self.peek_char(), Some(ch) if ch.is_ascii_alphabetic()) {
            self.index += 1;
        }
        self.input[start..self.index].to_string()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek_char(), Some(ch) if ch.is_whitespace()) {
            self.index += 1;
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.index..].chars().next()
    }

    fn remaining(&self) -> &str {
        &self.input[self.index..]
    }

    fn is_eof(&self) -> bool {
        self.index >= self.input.len()
    }
}
