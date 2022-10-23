use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinOp {
    And,
    Or,
    Xor,
    Impl,
    Leq,
}

#[derive(PartialEq, Eq)]
pub enum ParseError {
    MissingOperand,
    InvalidCharacter(char),
    UnbalancedExpression,
}

use ParseError::{InvalidCharacter, MissingOperand, UnbalancedExpression};

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MissingOperand => write!(f, "Missing operand"),
            InvalidCharacter(c) => write!(f, "Invalid character: '{}'", c),
            UnbalancedExpression => write!(f, "Unbalanced expression"),
        }
    }
}

impl TryFrom<char> for BinOp {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use BinOp::{And, Impl, Leq, Or, Xor};
        match c {
            '&' => Ok(And),
            '|' => Ok(Or),
            '^' => Ok(Xor),
            '=' => Ok(Leq),
            '>' => Ok(Impl),
            _ => Err(InvalidCharacter(c)),
        }
    }
}

impl From<BinOp> for char {
    fn from(op: BinOp) -> Self {
        use BinOp::{And, Impl, Leq, Or, Xor};
        match op {
            And => '&',
            Or => '|',
            Xor => '^',
            Impl => '>',
            Leq => '=',
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
