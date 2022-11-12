#[derive(Debug, Clone, Copy)]
pub enum Operator{
    Plus,
    Minus,
    Mul,
    Divide,
    Equal,
}

#[derive(Debug, Clone, Copy)]
pub enum MathFn {
    Sin,
    Cos,
    Tan,
}