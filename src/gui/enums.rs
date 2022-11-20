use std::convert;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
            Operator::Equal => unimplemented!(),
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
        (op as u8) as char
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum MathFn {
    Sin,
    Cos,
    Tan,
}
