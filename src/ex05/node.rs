use std::cell::RefCell;
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
    Val(Rc<RefCell<Var>>),
}

pub struct Tree {
    pub root: Node,
    pub variables: Vec<Rc<RefCell<Var>>>,
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
            Val(val) => write!(f, "{}", val.borrow().name),
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
        let variables: Vec<Rc<RefCell<Var>>> = ('A'..='Z')
            .map(|c| {
                Rc::new(RefCell::new(Var {
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

impl Node {
    pub fn nnf(self) -> Box<Node> {
        match self {
            Val(v) => Box::new(Val(v)),
            Binary { op, left, right } => match op {
                // Xor -> (!A & B ) | (A & !B)
                Xor => Binary {
                    op: Or,
                    left: Box::new(Binary {
                        op: And,
                        left: Box::new(Not {
                            operand: left.clone(),
                        }),
                        right: right.clone(),
                    }),
                    right: Box::new(Binary {
                        op: And,
                        left,
                        right: Box::new(Not { operand: right }),
                    }),
                }
                .nnf(),
                // Impl -> !A | B
                Impl => Binary {
                    op: Or,
                    left: Box::new(Not { operand: left }),
                    right,
                }
                .nnf(),
                // Leq == (A & B) | (!A & !B)
                Leq => Binary {
                    op: Or,
                    left: Box::new(Binary {
                        op: And,
                        left: left.clone(),
                        right: right.clone(),
                    }),
                    right: Box::new(Binary {
                        op: And,
                        left: Box::new(Not { operand: left }),
                        right: Box::new(Not { operand: right }),
                    }),
                }
                .nnf(),
                _ => Box::new(Binary {
                    op,
                    left: left.nnf(),
                    right: right.nnf(),
                }),
            },
            Not { operand } => match *operand {
                Val(v) => Box::new(Not {
                    operand: Box::new(Val(v)),
                }),
                Not { operand } => (*operand).nnf(),
                Binary { op, left, right } => match op {
                    // !(A & B) -> !A | !B
                    And => Binary {
                        op: Or,
                        left: Box::new(Not { operand: left }),
                        right: Box::new(Not { operand: right }),
                    }
                    .nnf(),
                    // !(A | B) -> !A & !B
                    Or => Binary {
                        op: And,
                        left: Box::new(Not { operand: left }),
                        right: Box::new(Not { operand: right }),
                    }
                    .nnf(),
                    // else, first convert to & or |, then call nnf on the result
                    _ => Not {
                        operand: Binary { op, left, right }.nnf(),
                    }
                    .nnf(),
                },
            },
        }
    }
}
