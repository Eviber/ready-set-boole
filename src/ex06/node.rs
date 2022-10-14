use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use BinOp::*;
use Literal::*;
use ParseError::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    And,
    Or,
    Xor,
    Impl,
    Leq,
}

#[derive(Clone, Copy)]
pub struct Variable {
    pub name: char,
    pub value: bool,
}

pub type VarCell = Rc<Cell<Variable>>;

#[derive(Clone)]
pub enum Literal {
    Binary { op: BinOp, children: Vec<Node> },
    Var(VarCell),
    Const(bool),
}

#[derive(Clone)]
pub struct Node {
    pub not: usize,
    pub literal: Literal,
}

pub struct Tree {
    pub root: Node,
    pub variables: Vec<VarCell>,
}

#[derive(PartialEq, Eq)]
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
            Const(val) => write!(f, "{}", *val as u8),
        }
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
        let variables: Vec<VarCell> = ('A'..='Z')
            .map(|c| {
                Rc::new(Cell::new(Variable {
                    name: c,
                    value: false,
                }))
            })
            .collect();

        for c in s.chars() {
            match c {
                '0' | '1' => stack.push(Node {
                    not: 0,
                    literal: Const(c == '1'),
                }),
                'A'..='Z' => stack.push(Node {
                    not: 0,
                    literal: Var(variables[c as usize - b'A' as usize].clone()),
                }),
                '!' => {
                    let operand = stack.pop().ok_or(MissingOperand)?;
                    stack.push(Node {
                        not: operand.not + 1,
                        literal: operand.literal,
                    });
                }
                _ => {
                    let tmp = stack.pop().ok_or(MissingOperand)?; // for the reverse pop order
                    let literal = Binary {
                        op: BinOp::try_from(c)?,
                        children: vec![stack.pop().ok_or(MissingOperand)?, tmp],
                    };
                    stack.push(Node { not: 0, literal });
                }
            }
        }
        if stack.len() == 1 {
            Ok(Tree {
                root: stack.pop().unwrap(),
                variables,
            })
        } else {
            Err(UnbalancedExpression)
        }
    }
}

fn new_binary(op: BinOp, children: Vec<Node>) -> Node {
    Node {
        not: 0,
        literal: Binary { op, children },
    }
}

// TODO: implement binary operations for node
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

fn leq(left: Node, right: Node) -> Node {
    new_binary(Leq, vec![left, right])
}

// not operator
impl std::ops::Not for Node {
    type Output = Node;
    fn not(self) -> Node {
        Node {
            not: self.not + 1,
            literal: self.literal,
        }
    }
}

#[derive(PartialEq, Eq)]
enum NodeCmp {
    Equal,
    NotEqual,
    Opposite,
}

impl Node {
    fn compare(&self, other: &Node) -> NodeCmp {
        match (&self.literal, &other.literal) {
            (Var(v1), Var(v2)) => {
                if v1.get().name == v2.get().name {
                    if self.not == other.not {
                        NodeCmp::Equal
                    } else {
                        NodeCmp::Opposite
                    }
                } else {
                    NodeCmp::NotEqual
                }
            }
            (Const(c1), Const(c2)) => {
                if c1 ^ (self.not % 2 == 1) == c2 ^ (other.not % 2 == 1) {
                    NodeCmp::Equal
                } else {
                    NodeCmp::Opposite
                }
            }
            (
                Binary {
                    op: op1,
                    children: children1,
                },
                Binary {
                    op: op2,
                    children: children2,
                },
            ) => {
                if op1 == op2 && children1.len() == children2.len() && self.not == other.not {
                    let mut children1 = children1.clone();
                    let mut children2 = children2.clone();
                    let cmp = |a: &Node, b: &Node| {
                        if let Var(v1) = &a.literal {
                            if let Var(v2) = &b.literal {
                                if v1.get().name == v2.get().name {
                                    return a.not < b.not;
                                }
                                return v1.get().name < v2.get().name;
                            }
                        }
                        false
                    };
                    children1.sort_by(|a, b| cmp(a, b).cmp(&false));
                    children2.sort_by(|a, b| cmp(a, b).cmp(&false));
                    for (a, b) in children1.iter().zip(children2.iter()) {
                        if a.compare(b) != NodeCmp::Equal {
                            return NodeCmp::NotEqual;
                        }
                    }
                    NodeCmp::Equal
                } else {
                    NodeCmp::NotEqual
                }
            }
            _ => NodeCmp::NotEqual,
        }
    }
}

impl Node {
    pub fn cnf(self) -> Node {
        let mut new = self.clone();
        new.not = self.not % 2;
        if new.not == 1 {
            match new.literal {
                Const(_) | Var(_) => new,
                Binary { op, children } => {
                    let left = children[0].clone();
                    let right = children[1].clone();
                    match op {
                        // !(A & B) -> !A | !B
                        And => (!left | !right).cnf(),
                        // !(A | B) -> !A & !B
                        Or => (!left & !right).cnf(),
                        // !(A = B) -> A ^ B
                        Leq => (left ^ right).cnf(),
                        // !(A ^ B) -> A = B
                        Xor => leq(left, right).cnf(),
                        // !(A > B) -> A & !B
                        Impl => (left & !right).cnf(),
                    }
                }
            }
        } else {
            match new.literal {
                Const(_) | Var(_) => new,
                Binary { op, children } => {
                    let left = children[0].clone();
                    let right = children[1].clone();
                    match op {
                        // Xor -> (A | B) & (!A | !B)
                        Xor => ((left.clone() | right.clone()) & (!left | !right)).cnf(),
                        // Impl -> !A | B
                        Impl => (!left | right).cnf(),
                        // Leq == (A | !B) & (!A | B)
                        Leq => ((left.clone() | !right.clone()) & (!left | right)).cnf(),
                        And => left.cnf() & right.cnf(),
                        Or => {
                            // recurse first to bring up any ANDs
                            let left = left.cnf();
                            let right = right.cnf();
                            if let Binary { op: And, children } = left.literal {
                                // (A & B) | C -> (A | C) & (B | C)
                                ((children[0].clone() | right.clone())
                                    & (children[1].clone() | right))
                                    .cnf()
                            } else if let Binary { op: And, children } = right.literal {
                                // A & (B | C) -> (A | B) & (A | C)
                                ((left.clone() | children[0].clone())
                                    & (left | children[1].clone()))
                                .cnf()
                            } else {
                                // if neither left nor right is an And, we're done
                                left | right
                            }
                        }
                    }
                }
            }
        }
    }

    // fn equals(&self, other: &Node) -> bool {
    //     match (self, other) {
    //         (Const(a), Const(b)) => a == b,
    //         (Var(a), Var(b)) => a.get().name == b.get().name,
    //         (
    //             Binary { op, left, right },
    //             Binary {
    //                 op: o,
    //                 left: l,
    //                 right: r,
    //             },
    //         ) => {
    //             if op == o {
    //                 if op == &Impl {
    //                     left.equals(l) && right.equals(r)
    //                 } else {
    //                     left.equals(l) && right.equals(r) || (left.equals(r) && right.equals(l))
    //                 }
    //             } else {
    //                 false
    //             }
    //         }
    //         (Not(a), Not(b)) => a.equals(b),
    //         _ => false,
    //     }
    // }

    pub fn simplify(self) -> Node {
        let mut new = self.clone();
        new.not = self.not % 2;
        match new.literal {
            Const(c) => Node {
                not: 0,
                literal: Const(c ^ (new.not == 1)),
            },
            Var(_) => new,
            Binary { op, children } => {
                let mut new_children = Vec::new();
                for child in children.clone() {
                    if let Binary { op: o, children: c } = child.clone().simplify().literal {
                        if op == o {
                            new_children.extend(c);
                        } else {
                            new_children.push(child.simplify());
                        }
                    } else {
                        new_children.push(child.simplify());
                    }
                }
                println!("children: {:?}", children);
                let mut children = new_children;
                for i in 0..children.len() {
                    for j in (i + 1)..children.len() {
                        if children.get(j).is_none() {
                            continue;
                        }
                        if let NodeCmp::Equal = children[i].compare(&children[j]) {
                            children.remove(j);
                        }
                    }
                }
                println!("new children: {:?}", children);
                let mut new_children: Vec<Node> = Vec::new();
                match op {
                    And => {
                        // iterate through children, while removing duplicates
                        // if any are false, return false
                        // if any are true, remove them
                        // if there are conflicting children, return false
                        for child in &children {
                            if let Const(c) = child.literal {
                                if c ^ (child.not == 1) {
                                    continue;
                                }
                                return Node {
                                    not: 0,
                                    literal: Const(false),
                                };
                            }
                            let mut to_add = true;
                            for new_child in &new_children {
                                match child.compare(new_child) {
                                    NodeCmp::Equal => {
                                        to_add = false;
                                        break;
                                    }
                                    NodeCmp::Opposite => {
                                        return Node {
                                            not: 0,
                                            literal: Const(false),
                                        };
                                    }
                                    NodeCmp::NotEqual => {}
                                }
                            }
                            if to_add {
                                new_children.push(child.clone());
                            }
                        }
                        match new_children.len() {
                            0 => Node {
                                not: 0,
                                literal: Const(true),
                            },
                            1 => new_children[0].clone(),
                            _ => Node {
                                not: 0,
                                literal: Binary {
                                    op: And,
                                    children: new_children,
                                },
                            },
                        }
                    }
                    Or => {
                        // iterate through children, while removing duplicates
                        // if any are true, return true
                        // if any are false, remove them
                        // if there are conflicting children, return true
                        for child in &children {
                            if let Const(c) = child.literal {
                                if c ^ (child.not == 1) {
                                    return Node {
                                        not: 0,
                                        literal: Const(true),
                                    };
                                }
                                continue;
                            }
                            let mut to_add = true;
                            for new_child in &new_children {
                                match child.compare(new_child) {
                                    NodeCmp::Equal => {
                                        to_add = false;
                                        break;
                                    }
                                    NodeCmp::Opposite => {
                                        return Node {
                                            not: 0,
                                            literal: Const(true),
                                        };
                                    }
                                    NodeCmp::NotEqual => {}
                                }
                            }
                            if to_add {
                                new_children.push(child.clone());
                            }
                        }
                        match new_children.len() {
                            0 => Node {
                                not: 0,
                                literal: Const(false),
                            },
                            1 => new_children[0].clone(),
                            _ => Node {
                                not: 0,
                                literal: Binary {
                                    op: Or,
                                    children: new_children,
                                },
                            },
                        }
                    }
                    Xor => {
                        // Xor is not associative, so it's a bit different here
                        // it should only have two children
                        // if they are equal, return false
                        // if they are opposite, return true
                        // if one is true, return the other negated
                        // if one is false, return the other
                        // otherwise, return the xor of the two
                        todo!();
                    }
                    Impl => todo!(),
                    Leq => todo!(),
                }
                // match op {
                //     And => Box::new(match (*left, *right) {
                //         (Const(false), _) | (_, Const(false)) => Const(false),
                //         (Const(true), right) => right,
                //         (left, Const(true)) => left,
                //         (left, right) => {
                //             if left.equals(&right) {
                //                 left
                //             } else {
                //                 Binary {
                //                     op,
                //                     left: Box::new(left),
                //                     right: Box::new(right),
                //                 }
                //             }
                //         }
                //     }),
                //     Or => Box::new(match (*left, *right) {
                //         (Const(true), _) | (_, Const(true)) => Const(true),
                //         (Const(false), right) => right,
                //         (left, Const(false)) => left,
                //         (left, right) => {
                //             if left.equals(&right) {
                //                 left
                //             } else {
                //                 Binary {
                //                     op,
                //                     left: Box::new(left),
                //                     right: Box::new(right),
                //                 }
                //             }
                //         }
                //     }),
                //     Xor => Box::new(match (*left, *right) {
                //         (Const(a), Const(b)) => Const(a ^ b),
                //         (Const(false), right) => right,
                //         (left, Const(false)) => left,
                //         (Const(true), right) => *(!right),
                //         (left, Const(true)) => *(!left),
                //         (left, right) => {
                //             if left.equals(&right) {
                //                 Const(false)
                //             } else {
                //                 Binary {
                //                     op,
                //                     left: Box::new(left),
                //                     right: Box::new(right),
                //                 }
                //             }
                //         }
                //     }),
                //     Leq => Box::new(match (*left, *right) {
                //         (Const(a), Const(b)) => Const(a == b),
                //         (Const(false), right) => *(!right),
                //         (left, Const(false)) => *(!left),
                //         (Const(true), right) => right,
                //         (left, Const(true)) => left,
                //         (left, right) => {
                //             if left.equals(&right) {
                //                 Const(true)
                //             } else {
                //                 Binary {
                //                     op,
                //                     left: Box::new(left),
                //                     right: Box::new(right),
                //                 }
                //             }
                //         }
                //     }),
                //     Impl => Box::new(match (*left, *right) {
                //         (Const(false), _) | (_, Const(true)) => Const(true),
                //         (Const(true), right) => right,
                //         (left, Const(false)) => *(!left),
                //         (left, right) => {
                //             if left.equals(&right) {
                //                 Const(true)
                //             } else {
                //                 Binary {
                //                     op,
                //                     left: Box::new(left),
                //                     right: Box::new(right),
                //                 }
                //             }
                //         }
                //     }),
                // }
            }
        }
    }
}
