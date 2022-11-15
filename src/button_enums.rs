#[derive(Debug, Clone, Copy)]

#[allow(dead_code)]
pub enum Operator{
    Plus,
    Minus,
    Mul,
    Divide,
    Equal,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum MathFn {
    Sin,
    Cos,
    Tan,
}