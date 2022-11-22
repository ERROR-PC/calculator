use std::convert;
use std::fmt;

/// Error occured while converting something into an operator
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct OperatorParseError {
    pub ch: char,
}

impl fmt::Display for OperatorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// Error occured while converting an expr to some type
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct ExprParseError {
    pub ch: char,
}

impl fmt::Display for ExprParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// Error occured while converting T into a token
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct TokenizationError<T>
where T:
    fmt::Debug + fmt::Display
{ pub cause: T }

impl<T: fmt::Debug + fmt::Display> fmt::Display for TokenizationError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Can not tokenize: {}", self.cause)
    }
}

impl<T: fmt::Debug + fmt::Display> std::error::Error for TokenizationError<T> {}
