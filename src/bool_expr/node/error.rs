use super::Token;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedCharacterError(char),
    MissingOperand(Token),
    UnbalancedExpression(Token, Token),
    EmptyExpression,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedCharacterError(c) => write!(f, "Unexpected character: '{}'", c),
            ParseError::MissingOperand(operator) => {
                write!(f, "Missing operand for operator {}", operator)
            }
            ParseError::UnbalancedExpression(lhs, rhs) => write!(
                f,
                "Unbalanced expression: no operator to apply to {lhs} and {rhs}"
            ),
            ParseError::EmptyExpression => write!(f, "Empty expression"),
        }
    }
}

impl std::error::Error for ParseError {}
