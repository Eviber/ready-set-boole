use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
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

#[derive(Clone, Copy)]
pub struct Var {
    pub name: char,
    pub value: bool,
}

#[derive(Clone)]
pub enum Node {
    Binary {
        op: BinOp,
        left: Box<Node>,
        right: Box<Node>,
    },
    Not {
        operand: Box<Node>,
    },
    Val(Rc<Cell<Var>>),
}

pub struct Tree {
    pub root: Node,
    pub variables: Vec<Rc<Cell<Var>>>,
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
            Binary { op, left, right } => write!(f, "{}{}{}", left, right, op),
            Not { operand } => write!(f, "{}!", operand),
            Val(val) => write!(f, "{}", val.get().name),
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

impl std::str::FromStr for Tree {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::with_capacity(s.len());
        let variables: Vec<Rc<Cell<Var>>> = ('A'..='Z')
            .map(|c| {
                Rc::new(Cell::new(Var {
                    name: c,
                    value: false,
                }))
            })
            .collect();

        for c in s.chars() {
            match c {
                'A'..='Z' => {
                    stack.push(Val(variables[c as usize - b'A' as usize].clone()));
                }
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
            Ok(Tree {
                root: stack.pop().unwrap(),
                variables,
            })
        }
    }
}

// TODO: implement binary operations for node
impl std::ops::BitOr for Box<Node> {
    type Output = Box<Node>;
    fn bitor(self, other: Box<Node>) -> Box<Node> {
        Box::new(Binary {
            op: Or,
            left: self,
            right: other,
        })
    }
}

impl std::ops::BitAnd for Box<Node> {
    type Output = Box<Node>;
    fn bitand(self, other: Box<Node>) -> Box<Node> {
        Box::new(Binary {
            op: And,
            left: self,
            right: other,
        })
    }
}

// not operator
impl std::ops::Not for Box<Node> {
    type Output = Box<Node>;
    fn not(self) -> Box<Node> {
        Box::new(Not { operand: self })
    }
}

impl std::ops::Not for Node {
    type Output = Box<Node>;
    fn not(self) -> Box<Node> {
        Box::new(Not {
            operand: Box::new(self),
        })
    }
}

impl Node {
    pub fn nnf(self) -> Box<Node> {
        match self {
            Val(v) => Box::new(Val(v)),
            Binary { op, left, right } => match op {
                // Xor -> (!A & B ) | (A & !B)
                Xor => ((left.clone() & !right.clone()) | (!left & right)).nnf(),
                // Impl -> !A | B
                Impl => (!left | right).nnf(),
                // Leq == (A & B) | (!A & !B)
                Leq => ((left.clone() & right.clone()) | (!left & !right)).nnf(),
                And => left.nnf() & right.nnf(),
                Or => left.nnf() | right.nnf(),
            },
            Not { operand } => match *operand {
                Val(v) => !Val(v),
                Not { operand } => (*operand).nnf(),
                Binary { op, left, right } => match op {
                    // !(A & B) -> !A | !B
                    And => (!left | !right).nnf(),
                    // !(A | B) -> !A & !B
                    Or => (!left & !right).nnf(),
                    // else, first convert to & or |, then call nnf on the result
                    _ => (!Binary { op, left, right }.nnf()).nnf(),
                },
            },
        }
    }
}
