use std::convert;

#[derive(Debug, Clone, Copy, Default)]
pub struct OperatorParseError {
    pub ch: char,
}

impl std::fmt::Display for OperatorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The char ")?;
        self.ch.fmt(f)?;
        write!(f, " is not a mathematical operator")
    }
}

impl std::error::Error for OperatorParseError {}

impl convert::From<ExprParseError> for OperatorParseError {
    fn from(err: ExprParseError) -> Self {
        Self { ch: err.ch }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExprParseError {
    pub ch: char,
}

impl std::fmt::Display for ExprParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The char ")?;
        self.ch.fmt(f)?;
        write!(f, " caused an error during parsing")
    }
}

impl std::error::Error for ExprParseError {}

impl convert::From<OperatorParseError> for ExprParseError {
    fn from(err: OperatorParseError) -> Self {
        Self { ch: err.ch }
    }
}
