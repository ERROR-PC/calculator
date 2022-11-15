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

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum MathFn {
    Sin,
    Cos,
    Tan,
}
