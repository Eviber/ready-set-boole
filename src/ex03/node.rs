use std::fmt;
use BinOp::*;
use Node::*;
use ParseError::*;

#[derive(Clone, Copy)]
pub enum BinOp {
    And,
    Or,
    Xor,
    Impl,
    Leq,
}

pub enum Node {
    Binary {
        op: BinOp,
        left: Box<Node>,
        right: Box<Node>,
    },
    Not {
        operand: Box<Node>,
    },
    Val(bool),
}

#[derive(PartialEq)]
pub enum ParseError {
    MissingOperand,
    InvalidCharacter(char),
    UnbalancedExpression,
}

impl TryFrom<char> for BinOp {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Binary { op, left, right } => write!(f, "({} {} {})", left, op, right),
            Not { operand } => write!(f, "!{}", operand),
            Val(val) => write!(f, "{}", *val as u8),
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MissingOperand => write!(f, "Missing operand"),
            InvalidCharacter(c) => write!(f, "Invalid character: '{}'", c),
            UnbalancedExpression => write!(f, "Unbalanced expression"),
        }
    }
}

impl std::str::FromStr for Node {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::with_capacity(42);
        for c in s.chars() {
            match c {
                '0' => stack.push(Val(false)),
                '1' => stack.push(Val(true)),
                '!' => {
                    let operand = stack.pop().ok_or(MissingOperand)?;
                    stack.push(Not {
                        operand: Box::new(operand),
                    });
                }
                _ => {
                    let op = c.try_into()?; // BinOp or returns InvalidCharacter
                    let right = stack.pop().ok_or(MissingOperand)?;
                    let left = stack.pop().ok_or(MissingOperand)?;
                    stack.push(Binary {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    });
                }
            }
        }
        if stack.len() != 1 {
            Err(UnbalancedExpression)
        } else {
            Ok(stack.pop().unwrap())
        }
    }
}

impl Node {
    fn eval(self) -> bool {
        bool::from(self)
    }
}

impl From<Node> for bool {
    fn from(node: Node) -> Self {
        match node {
            Val(x) => x,
            Not { operand } => !operand.eval(),
            Binary { op, left, right } => match op {
                And => left.eval() && right.eval(),
                Or => left.eval() || right.eval(),
                Xor => left.eval() ^ right.eval(),
                Impl => !left.eval() || right.eval(),
                Leq => left.eval() == right.eval(),
            },
        }
    }
}
