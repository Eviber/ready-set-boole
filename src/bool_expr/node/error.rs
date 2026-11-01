#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedCharacterError(char),
    MissingOperand(super::Token),
    UnbalancedExpression,
    EmptyExpression,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedCharacterError(c) => write!(f, "{}", c),
            ParseError::MissingOperand(operator) => write!(f, "{}", operator),
            ParseError::UnbalancedExpression => write!(f, "unbalanced expression"),
            ParseError::EmptyExpression => write!(f, "empty expression"),
        }
    }
}

impl std::error::Error for ParseError {}
