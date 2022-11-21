use std::convert;
use num_complex::Complex64;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Pressed {
    Num(u8),
    Op(Operator),
    Const(Complex64),
    Func(MathFn),
    Keyboard(iced::keyboard::Event),
}

impl convert::TryFrom<char> for Pressed {
    type Error = crate::errors::ExprParseError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        use std::f64::consts::{PI, E};

        if let Ok(op) = Operator::try_from(ch) {
            return Ok(Pressed::Op(op));
        }
        match ch.to_ascii_lowercase() {
            '0'..='9' => Ok(Pressed::Num(ch as u8 - crate::ASCII_OF_0)),
            'i' | 'j' => Ok(Pressed::Const(Complex64::i())),
            'p' | 'π' | 'Π' | 'ϖ' => Ok(Pressed::Const(PI.into())),
            'e' => Ok(Pressed::Const(E.into())),
            _ => Err(Self::Error { ch }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
pub enum Operator{
    Plus = b'+',
    Minus = b'-',
    Mul = b'\xD7', // × symbol for mul
    Divide = b'\xF7', // ÷ symbol for division
    Equal = b'=',
}

impl Operator {
    pub fn precedence(self) -> u32 {
        match self {
            Operator::Plus => 2,
            Operator::Minus => 2,
            Operator::Mul => 3,
            Operator::Divide => 3,
            Operator::Equal => 0,
        }
    }
}

impl convert::From<Operator> for u8 {
    #[inline]
    fn from(op: Operator) -> Self {
        op as u8
    }
}

impl convert::From<Operator> for char {
    #[inline]
    fn from(op: Operator) -> Self {
        u8::from(op) as char
    }
}

impl convert::TryFrom<char> for Operator {
    type Error = crate::errors::OperatorParseError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch.to_ascii_lowercase() {
            '+' => Ok(Operator::Plus),
            '-' => Ok(Operator::Minus),
            '*' | 'x' | '×' | '⋅' => Ok(Operator::Mul),
            '/' | '÷' | '\\' => Ok(Operator::Divide),
            '=' | '\r' | '\n' => Ok(Operator::Equal),
            _ => Err(Self::Error { ch }),
        }
    }
}

impl std::fmt::Display for Operator {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch: char = (*self).into();
        write!(f, "{}", ch)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[allow(dead_code)]
pub enum MathFn {
    Sin,
    Cos,
    Tan,
}
