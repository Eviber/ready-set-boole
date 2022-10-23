use super::{BinOp, Binary, Cell, Const, Node, Rc, Var, fmt};

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
