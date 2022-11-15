#[derive(Debug, Clone, Copy, Default)]
pub struct OperatorParseError { pub ch: char }

impl std::fmt::Display for OperatorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The char ")?;
        self.ch.fmt(f)?;
        write!(f, " is not a mathematical operator")
    }
}

impl std::error::Error for OperatorParseError {}