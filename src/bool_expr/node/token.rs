use super::ParseError;

use BinOpToken::*;
use Token::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    BinOp(BinOpToken),
    Not,
    Literal(bool),
    Variable(char),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOpToken {
    And,
    Or,
    Xor,
    Imply,
    Eq,
}

impl TryFrom<char> for Token {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A'..='Z' => Ok(Variable(c)),
            '0' => Ok(Literal(false)),
            '1' => Ok(Literal(true)),
            '!' => Ok(Not),
            '&' => Ok(BinOp(And)),
            '|' => Ok(BinOp(Or)),
            '^' => Ok(BinOp(Xor)),
            '>' => Ok(BinOp(Imply)),
            '=' => Ok(BinOp(Eq)),
            c => Err(ParseError::UnexpectedCharacterError(c)),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Variable(c) => *c,
            Literal(false) => '0',
            Literal(true) => '1',
            Not => '!',
            BinOp(And) => '&',
            BinOp(Or) => '|',
            BinOp(Xor) => '^',
            BinOp(Imply) => '>',
            BinOp(Eq) => '=',
        };
        write!(f, "{c}")
    }
}

pub(super) fn tokenize(s: &str) -> impl Iterator<Item = Result<Token, ParseError>> + use<'_> {
    s.chars().map(Token::try_from)
}
