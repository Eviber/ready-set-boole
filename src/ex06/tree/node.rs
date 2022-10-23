pub mod binop;
pub use binop::BinOp;
use binop::BinOp::{And, Impl, Leq, Or, Xor};
use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use Literal::{Binary, Const, Var};

#[derive(Clone, Copy, Eq)]
pub struct Variable {
    pub name: char,
    pub value: bool,
}

pub type VarCell = Rc<Cell<Variable>>;

#[derive(Clone, Eq)]
pub enum Literal {
    Binary { op: BinOp, children: Vec<Node> },
    Var(VarCell),
    Const(bool),
}

#[derive(Clone, Eq, PartialOrd, Ord)]
pub struct Node {
    pub not: usize,
    pub literal: Literal,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Variable) -> bool {
        self.name == other.name
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match (self, other) {
            (
                Binary { op, children },
                Binary {
                    op: op2,
                    children: children2,
                },
            ) => {
                // sort childrens to compare them
                let mut children = children.clone();
                let mut children2 = children2.clone();
                children.sort();
                children2.sort();
                op == op2 && children == children2
            }
            (Var(var1), Var(var2)) => var1.get().name == var2.get().name,
            (Const(b1), Const(b2)) => b1 == b2,
            _ => false,
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Literal) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (
                Binary { op, children },
                Binary {
                    op: op2,
                    children: children2,
                },
            ) => {
                // sort childrens to compare them
                let mut children = children.clone();
                let mut children2 = children2.clone();
                children.sort();
                children2.sort();
                match op.cmp(op2) {
                    std::cmp::Ordering::Equal => children.partial_cmp(&children2),
                    ord => Some(ord),
                }
            }
            (Var(var1), Var(var2)) => var1.get().name.partial_cmp(&var2.get().name),
            (Const(b1), Const(b2)) => b1.partial_cmp(b2),
            _ => None,
        }
    }
}

impl Ord for Literal {
    fn cmp(&self, other: &Literal) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => std::cmp::Ordering::Equal,
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Binary { op, children } => {
                for child in children {
                    write!(f, "{}", child)?;
                }
                // write the operator one time less than the number of children
                write!(f, "{}", op.to_string().repeat(children.len() - 1))
            }
            Var(val) => write!(f, "{}", val.get().name),
            Const(val) => write!(f, "{}", u8::from(*val)),
        }
    }
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
