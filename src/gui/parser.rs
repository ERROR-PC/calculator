use std::convert;

use num_complex::Complex64;
use crate::gui::{Operator, MathFn};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Op(Operator),
    Fn(MathFn),
    Num(Complex64),
}

impl convert::TryFrom<&str> for Token {
    type Error = crate::errors::ExprParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        for ch in text.chars() {
            if let Ok(num) = <char as TryInto<u8>>::try_into(ch) {
                return Ok(Token::Num((num as f64).into()));
            }

            if let Ok(op) = ch.try_into() {
                return Ok(Token::Op(op));
            }
        }

        todo!()
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Parser {
    out: Vec<Token>,
    op: Vec<Token>,
}

impl convert::TryFrom<&str> for Parser {
    type Error = crate::errors::ExprParseError;

    fn try_from(expr: &str) -> Result<Self, Self::Error> {
        let mut res = Self::default();
        let mut temp_str = String::new();
        for (i, ch) in expr.char_indices() {
            if ch.is_numeric() {
                temp_str.push(ch);
                if let Some(next_ch) = expr.get(i + 1..=i + 1) {
                    let next_ch = next_ch.chars().next().expect("Unfalliable error occured");
                    temp_str.push(next_ch);
                }
                else {
                    res.out.push(temp_str.try_into().expect("Sad"));
                }
            }
        }

        todo!()
    }
}