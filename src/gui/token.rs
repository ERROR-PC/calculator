use std::convert;

use num_complex::Complex64;

use crate::errors::TokenizationError;
use super::enums::{Pressed, Operator};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Num(Complex64),
    Op(Operator),
}

impl Token {
    /// If it is a Num variant, the complex number is returned
    ///
    /// # Panics
    /// Will panic if it is an op variant
    pub fn unwrap_complex(self) -> Complex64 {
        if let Token::Num(n) = self { n }
        else { panic!("Cannot unwrap an op variant into a number") }
    }

    /// If it is an Op variant, the operator is returned
    ///
    /// # Panics
    /// Will panic if it is a num variant
    pub fn unwrap_op(self) -> Operator {
        if let Token::Op(op) = self { op }
        else { panic!("Cannot unwrap an op variant into a number") }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(num) => {
                if num.im == 0.0 {
                    write!(f, "{}", num.re)
                } else if num.re == 0.0 {
                    write!(f, "{}i", num.im)
                } else {
                    write!(f, "{}", num)
                }
            }
            Self::Op(op) => write!(f, "{}", op),
        }
    }
}

impl std::str::FromStr for Token {
    type Err = TokenizationError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = Complex64::from_str(s) {
            Ok(Token::Num(num))
        }
        else if s.len() != 1 {
            Err(TokenizationError { cause: s.to_string() })
        }
        // only one char
        else {
            let ch: char = s.chars().next().expect("The string should be a single character");
            let Ok(op) = ch.try_into() else {
                return Err(Self::Err { cause: s.to_string() });
            };
            Ok(Token::Op(op))
        }
    }
}

// Why did I implement this?
/// Convert from a user input to a token
impl convert::TryFrom<Pressed> for Token {
    type Error = TokenizationError<Pressed>;

    fn try_from(value: Pressed) -> Result<Self, Self::Error> {
        match value {
            Pressed::Num(num) => Ok(Token::Num((num as f64).into())),
            Pressed::Op(op) => Ok(Token::Op(op)),
            Pressed::Const(num) => Ok(Token::Num(num)),
            _ => Err(Self::Error { cause: value }),
        }
    }
}

impl convert::From<Complex64> for Token {
    #[inline]
    fn from(num: Complex64) -> Self {
        Token::Num(num)
    }
}

impl convert::From<Operator> for Token {
    #[inline]
    fn from(op: Operator) -> Self {
        Token::Op(op)
    }
}
