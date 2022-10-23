pub mod binop;
mod literal;

pub use binop::BinOp;
use binop::BinOp::{And, Impl, Leq, Or, Xor};
pub use literal::*;
use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use Literal::{Binary, Const, Var};

#[derive(Clone, Eq, PartialOrd, Ord)]
pub struct Node {
    pub not: usize,
    pub literal: Literal,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.not == other.not && self.literal == other.literal
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.literal)?;
        if self.not > 0 {
            write!(f, "{}", "!".repeat(self.not as usize))
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn new_binary(op: BinOp, children: Vec<Node>) -> Node {
    Node {
        not: 0,
        literal: Binary { op, children },
    }
}

impl std::ops::BitOr for Node {
    type Output = Node;
    fn bitor(self, other: Node) -> Node {
        new_binary(Or, vec![self, other])
    }
}

impl std::ops::BitAnd for Node {
    type Output = Node;
    fn bitand(self, other: Node) -> Node {
        new_binary(And, vec![self, other])
    }
}

impl std::ops::BitXor for Node {
    type Output = Node;
    fn bitxor(self, other: Node) -> Node {
        new_binary(Xor, vec![self, other])
    }
}

impl std::ops::Not for Node {
    type Output = Node;
    fn not(self) -> Node {
        Node {
            not: self.not + 1,
            literal: self.literal,
        }
    }
}

impl Node {
    pub fn eval(&self) -> bool {
        let res = match &self.literal {
            Const(c) => *c,
            Var(v) => v.get().value,
            Binary { op, children } => {
                let left = children[0].eval();
                let right = children[1].eval();
                match op {
                    And => left && right,
                    Or => left || right,
                    Impl => !left || right,
                    Leq => left == right,
                    Xor => left ^ right,
                }
            }
        };
        res ^ (self.not % 2 == 1)
    }
}
