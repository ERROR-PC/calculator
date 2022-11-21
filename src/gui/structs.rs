use num_complex::Complex64;
use crate::gui::enums::Operator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Num(Complex64),
    Op(Operator),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(num) => {
                if num.im == 0.0 {
                    write!(f, "{}", num.re)
                }
                else {
                    write!(f, "{}", num)
                }
            },
            Self::Op(op) => write!(f, "{}", op),
        }
    }
}