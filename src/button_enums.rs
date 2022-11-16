use std::convert;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Operator{
    Plus = b'+',
    Minus = b'-',
    Mul = b'\xD7', // × symbol for mul
    Divide = b'\xF7', // ÷ symbol for division
    Equal = b'=',
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
            '/' | '÷' => Ok(Operator::Divide),
            '=' | '\r' | '\n' => Ok(Operator::Equal),
            _ => Err(Self::Error { ch }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum MathFn {
    Sin,
    Cos,
    Tan,
}
