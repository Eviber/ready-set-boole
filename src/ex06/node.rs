use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use BinOp::*;
use Literal::*;
use ParseError::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinOp {
    And,
    Or,
    Xor,
    Impl,
    Leq,
}

#[derive(Clone, Copy, Eq)]
pub struct Variable {
    pub name: char,
    pub value: bool,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Variable) -> bool {
        self.name == other.name
    }
}

pub type VarCell = Rc<Cell<Variable>>;

#[derive(Clone, Eq)]
pub enum Literal {
    Binary { op: BinOp, children: Vec<Node> },
    Var(VarCell),
    Const(bool),
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
        if self.not == other.not {
            if self.literal == other.literal {
                NodeCmp::Equal
            } else {
                NodeCmp::NotEqual
            }
        } else if self.literal == other.literal {
            NodeCmp::Opposite
        } else {
            NodeCmp::NotEqual
        }
    }
}

pub fn get_table(input: &str, vars: &str) -> Vec<bool> {
    let tree = input.parse::<Tree>().expect("input is valid");
    let var_list: Vec<char> = ('A'..='Z').filter(|&c| vars.contains(c)).collect();
    let mut res = Vec::with_capacity(1 << var_list.len());
    for i in 0..(1 << var_list.len()) {
        for (j, v) in var_list.iter().enumerate() {
            let j = var_list.len() - j - 1;
            let bit = (i >> j) & 1;
            tree.set_var(*v, bit == 1);
        }
        res.push(tree.root.eval());
    }
    res
}

impl Tree {
    fn set_var(&self, name: char, value: bool) {
        self.variables[name as usize - 'A' as usize].set(Variable { name, value });
    }
}

impl Node {
    fn eval(&self) -> bool {
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum OptionBool {
    True,
    False,
    DontCare,
}

impl fmt::Debug for OptionBool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OptionBool::True => write!(f, "1"),
            OptionBool::False => write!(f, "0"),
            OptionBool::DontCare => write!(f, "-"),
        }
    }
}

impl Tree {
    pub fn cnf(&self) -> Tree {
        // Using the Quine-McCluskey algorithm
        // https://en.wikipedia.org/wiki/Quine%E2%80%93McCluskey_algorithm
        // https://electronics.stackexchange.com/questions/520513/can-quine-mccluskey-method-be-used-for-product-of-sum-simplification

        // Step 1: generate truth table
        let expr = self.root.to_string();
        let table = get_table(&expr, &expr);
        let bit_width = (table.len() - 1).count_ones() as usize;
        println!("table: {:?}", table);
        println!("bit_width: {}", bit_width);
        // we only need to look at the zero rows
        let false_rows: Vec<usize> = table
            .iter()
            .enumerate()
            .filter(|(_, &b)| !b)
            .map(|(i, _)| i)
            .collect();
        println!("false_rows: {:?}", false_rows);
        // Step 2: sort rows by number of 0s, from least to most
        let groups: Vec<Vec<Vec<OptionBool>>> = (0..=bit_width)
            .map(|i| {
                false_rows
                    .iter()
                    .rev()
                    .filter(|r| r.count_ones() as usize == bit_width - i)
                    .map(|&r| {
                        let mut v = Vec::with_capacity(bit_width);
                        for j in 0..bit_width {
                            v.push(if (r >> j) & 1 == 1 {
                                OptionBool::True
                            } else {
                                OptionBool::False
                            });
                        }
                        v
                    })
                    .collect()
            })
            .collect();
        println!("groups: {:?}", groups);
        // Step 3: generate prime implicants by combining rows
        let mut prime_implicants = loop {
            let mut prime_implicants = Vec::new();
            let mut stop = true;
            for i in 0..groups.len() - 1 {
                let mut new_prime_implicants: Vec<Vec<OptionBool>> = Vec::new();
                for j in 0..groups[i].len() {
                    for k in 0..groups[i + 1].len() {
                        let mut diff = 0;
                        let mut diff_index = 0;
                        for l in 0..groups[i][j].len() {
                            if groups[i][j][l] != groups[i + 1][k][l] {
                                diff += 1;
                                diff_index = l;
                            }
                        }
                        if diff == 1 {
                            let mut new_prime_implicant = groups[i][j].clone();
                            new_prime_implicant[diff_index] = OptionBool::DontCare;
                            new_prime_implicants.push(new_prime_implicant);
                            stop = false;
                        }
                    }
                }
                prime_implicants.append(&mut new_prime_implicants);
            }
            if stop {
                break prime_implicants;
            }
            groups = prime_implicants;
        };
        println!("prime_implicants: {:?}", prime_implicants);
        // Step 4: remove redundant prime implicants
        let mut essential_prime_implicants: Vec<Vec<OptionBool>> = Vec::new();
        while !prime_implicants.is_empty() {
            let mut new_prime_implicants: Vec<Vec<OptionBool>> = Vec::new();
            for i in 0..prime_implicants.len() {
                let mut covered = false;
                for j in 0..prime_implicants.len() {
                    if i == j {
                        continue;
                    }
                    let mut covered_count = 0;
                    for k in 0..prime_implicants[i].len() {
                        if prime_implicants[i][k] == OptionBool::DontCare
                            || prime_implicants[j][k] == OptionBool::DontCare
                        {
                            continue;
                        }
                        if prime_implicants[i][k] == prime_implicants[j][k] {
                            covered_count += 1;
                        }
                    }
                    if covered_count == prime_implicants[i].len() {
                        covered = true;
                        break;
                    }
                }
                if !covered {
                    essential_prime_implicants.push(prime_implicants[i].clone());
                } else {
                    new_prime_implicants.push(prime_implicants[i].clone());
                }
            }
            prime_implicants = new_prime_implicants;
        }
        println!(
            "essential_prime_implicants: {:?}",
            essential_prime_implicants
        );
        // Step 5: generate final expression
        todo!()
    }
}

impl Node {
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
                for child in children {
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
