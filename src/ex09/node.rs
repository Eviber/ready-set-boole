use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use BinOp::*;
use Node::*;
use ParseError::*;
use Set::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    And,
    Or,
    Xor,
    Impl,
    Leq,
}

#[derive(Clone)]
pub struct Variable {
    pub name: char,
    pub value: Vec<i32>,
}

#[derive(Clone)]
enum Set {
    Positive(Vec<i32>),
    Negative(Vec<i32>),
}

pub type VarCell = Rc<RefCell<Variable>>;

#[derive(Clone)]
pub enum Node {
    Binary {
        op: BinOp,
        left: Box<Node>,
        right: Box<Node>,
    },
    Not(Box<Node>),
    Var(VarCell),
    Const(bool),
}

pub struct Tree {
    pub root: Node,
    pub variables: Vec<VarCell>,
    varlist: Vec<char>,
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Binary { op, left, right } => write!(f, "{}{}{}", left, right, op),
            Not(operand) => write!(f, "{}!", operand),
            Var(val) => write!(f, "{}", val.borrow().name),
            Const(val) => write!(f, "{}", *val as u8),
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
        let variables: Vec<VarCell> = ('A'..='Z')
            .map(|c| {
                Rc::new(RefCell::new(Variable {
                    name: c,
                    value: vec![],
                }))
            })
            .collect();
        let mut varlist = [false; 26];

        for c in s.chars() {
            match c {
                '0' | '1' => stack.push(Node::Const(c == '1')),
                'A'..='Z' => {
                    let i = c as usize - 'A' as usize;
                    stack.push(Var(variables[i].clone()));
                    varlist[i] = true;
                }
                '!' => {
                    let operand = stack.pop().ok_or(MissingOperand)?;
                    stack.push(Not(Box::new(operand)));
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
                varlist: varlist
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &v)| {
                        if v {
                            Some((i as u8 + b'A') as char)
                        } else {
                            None
                        }
                    })
                    .collect(),
            })
        }
    }
}

fn join(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    a.extend(b);
    a.sort_unstable();
    a.dedup();
    a
}

fn remove(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    a.iter().filter(|&val| !b.contains(val)).copied().collect()
}

fn intersect(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    a.retain(|val| b.contains(val));
    a
}

impl std::ops::BitOr for Set {
    type Output = Set;

    fn bitor(self, other: Set) -> Set {
        match (self, other) {
            (Positive(vec1), Positive(vec2)) => Positive(join(vec1, vec2)),
            (Negative(vec1), Negative(vec2)) => Negative(join(vec1, vec2)),
            (Positive(pvec), Negative(nvec)) | (Negative(nvec), Positive(pvec)) => {
                Negative(remove(nvec, pvec))
            }
        }
    }
}

impl std::ops::BitAnd for Set {
    type Output = Set;
    fn bitand(self, other: Set) -> Set {
        match (self, other) {
            (Positive(vec1), Positive(vec2)) => Positive(intersect(vec1, vec2)),
            (Negative(vec1), Negative(vec2)) => Negative(join(vec1, vec2)),
            (Positive(pvec), Negative(nvec)) | (Negative(nvec), Positive(pvec)) => {
                Negative(remove(pvec, nvec))
            }
        }
    }
}

impl std::ops::Not for Set {
    type Output = Set;
    fn not(self) -> Set {
        match self {
            Positive(a) => Negative(a),
            Negative(a) => Positive(a),
        }
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

impl std::ops::BitXor for Set {
    type Output = Set;
    fn bitxor(self, other: Set) -> Set {
        match (self, other) {
            (Positive(mut a), Positive(mut b)) => {
                let mut c = a
                    .iter()
                    .filter(|&x| !b.contains(x))
                    .copied()
                    .collect::<Vec<_>>();
                c.append(&mut b.iter().filter(|&x| !a.contains(x)).cloned().collect());
                Positive(c)
            }
            (Positive(mut a), Negative(mut b)) => {
                Positive(a.iter().filter(|&x| b.contains(x)).cloned().collect())
            }
            (Negative(mut a), Positive(mut b)) => {
                Positive(b.iter().filter(|&x| a.contains(x)).cloned().collect())
            }
            (Negative(mut a), Negative(mut b)) => {
                Negative(a.iter().filter(|&x| b.contains(x)).cloned().collect())
            }
        }
    }
}

impl std::ops::BitXor for Box<Node> {
    type Output = Box<Node>;
    fn bitxor(self, other: Box<Node>) -> Box<Node> {
        Box::new(Binary {
            op: Xor,
            left: self,
            right: other,
        })
    }
}

fn leq(left: Box<Node>, right: Box<Node>) -> Box<Node> {
    Box::new(Binary {
        op: Leq,
        left,
        right,
    })
}

impl std::ops::Not for Box<Node> {
    type Output = Box<Node>;
    fn not(self) -> Box<Node> {
        Box::new(Not(self))
    }
}

impl std::ops::Not for Node {
    type Output = Box<Node>;
    fn not(self) -> Box<Node> {
        Box::new(Not(Box::new(self)))
    }
}

impl Tree {
    fn set_vec(&self, name: char, vec: Vec<i32>) {
        self.variables[name as usize - 'A' as usize]
            .borrow_mut()
            .value = vec;
    }

    pub fn eval_set(&self, sets: Vec<Vec<i32>>) -> Vec<i32> {}
}

impl Node {
    pub fn eval_set(&self) -> Set {
        match self {
            Const(c) => unreachable!("Const nodes should not be evaluated"),
            Var(v) => Positive(v.borrow().value.clone()),
            Not(n) => !n.eval_set(),
            Binary { op, left, right } => match op {
                And => left.eval_set() & right.eval_set(),
                Or => left.eval_set() | right.eval_set(),
                Impl => !left.eval_set() | right.eval_set(),
                Leq => left.eval_set() == right.eval_set(),
                Xor => left.eval_set() ^ right.eval_set(),
            },
        }
    }

    pub fn cnf(self) -> Box<Node> {
        match self {
            Const(val) => Box::new(Const(val)),
            Var(v) => Box::new(Var(v)),
            Binary { op, left, right } => match op {
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
                    if let Binary {
                        op: And,
                        left: ll,
                        right: lr,
                    } = *left
                    {
                        // (A & B) | C -> (A | C) & (B | C)
                        ((ll | right.clone()) & (lr | right)).cnf()
                    } else if let Binary {
                        op: And,
                        left: rl,
                        right: rr,
                    } = *right
                    {
                        // A & (B | C) -> (A | B) & (A | C)
                        ((left.clone() | rl) & (left | rr)).cnf()
                    } else {
                        // if neither left nor right is an And, we're done
                        left | right
                    }
                }
            },
            Not(operand) => match *operand {
                Const(val) => Box::new(Const(!val)),
                Var(v) => !Var(v),
                Not(operand) => (*operand).cnf(),
                Binary { op, left, right } => match op {
                    // !(A & B) -> !A | !B
                    And => (!left | !right).cnf(),
                    // !(A | B) -> !A & !B
                    Or => (!left & !right).cnf(),
                    // !(A = B) -> A ^ B
                    Leq => (left ^ right).cnf(),
                    // !(A ^ B) -> A = B
                    Xor => leq(left, right).cnf(),
                    // else, first convert to & or |, then call cnf on the result
                    _ => (!Binary { op, left, right }.cnf()).cnf(),
                },
            },
        }
    }

    fn equals(&self, other: &Node) -> bool {
        match (self, other) {
            (Const(a), Const(b)) => a == b,
            (Var(a), Var(b)) => a.get().name == b.get().name,
            (
                Binary { op, left, right },
                Binary {
                    op: o,
                    left: l,
                    right: r,
                },
            ) => {
                if op == o {
                    if op != &Impl {
                        left.equals(l) && right.equals(r) || (left.equals(r) && right.equals(l))
                    } else {
                        left.equals(l) && right.equals(r)
                    }
                } else {
                    false
                }
            }
            (Not(a), Not(b)) => a.equals(b),
            _ => false,
        }
    }

    pub fn simplify(self) -> Box<Node> {
        match self {
            Const(val) => Box::new(Const(val)),
            Var(v) => Box::new(Var(v)),
            Not(n) => match *n {
                Const(val) => Box::new(Const(!val)),
                Var(v) => !Var(v),
                Not(n) => (*n).simplify(),
                Binary { op, left, right } => !Binary { op, left, right }.simplify(),
            },
            Binary { op, left, right } => {
                let left = left.simplify();
                let right = right.simplify();
                match op {
                    And => Box::new(match (*left, *right) {
                        (Const(false), _) => Const(false),
                        (_, Const(false)) => Const(false),
                        (Const(true), right) => right,
                        (left, Const(true)) => left,
                        (left, right) => {
                            if left.equals(&right) {
                                left
                            } else {
                                Binary {
                                    op,
                                    left: Box::new(left),
                                    right: Box::new(right),
                                }
                            }
                        }
                    }),
                    Or => Box::new(match (*left, *right) {
                        (Const(true), _) => Const(true),
                        (_, Const(true)) => Const(true),
                        (Const(false), right) => right,
                        (left, Const(false)) => left,
                        (left, right) => {
                            if left.equals(&right) {
                                left
                            } else {
                                Binary {
                                    op,
                                    left: Box::new(left),
                                    right: Box::new(right),
                                }
                            }
                        }
                    }),
                    Xor => Box::new(match (*left, *right) {
                        (Const(a), Const(b)) => Const(a ^ b),
                        (Const(false), right) => right,
                        (left, Const(false)) => left,
                        (Const(true), right) => *(!right),
                        (left, Const(true)) => *(!left),
                        (left, right) => {
                            if left.equals(&right) {
                                Const(false)
                            } else {
                                Binary {
                                    op,
                                    left: Box::new(left),
                                    right: Box::new(right),
                                }
                            }
                        }
                    }),
                    Leq => Box::new(match (*left, *right) {
                        (Const(a), Const(b)) => Const(a == b),
                        (Const(false), right) => *(!right),
                        (left, Const(false)) => *(!left),
                        (Const(true), right) => right,
                        (left, Const(true)) => left,
                        (left, right) => {
                            if left.equals(&right) {
                                Const(true)
                            } else {
                                Binary {
                                    op,
                                    left: Box::new(left),
                                    right: Box::new(right),
                                }
                            }
                        }
                    }),
                    Impl => Box::new(match (*left, *right) {
                        (Const(false), _) => Const(true),
                        (_, Const(true)) => Const(true),
                        (Const(true), right) => right,
                        (left, Const(false)) => *(!left),
                        (left, right) => {
                            if left.equals(&right) {
                                Const(true)
                            } else {
                                Binary {
                                    op,
                                    left: Box::new(left),
                                    right: Box::new(right),
                                }
                            }
                        }
                    }),
                }
            }
        }
    }
}
