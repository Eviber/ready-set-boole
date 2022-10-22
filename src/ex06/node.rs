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

pub fn get_table(input: &str, expr: &str) -> Vec<bool> {
    let tree = input.parse::<Tree>().expect("input is valid");
    let var_list: Vec<char> = ('A'..='Z').filter(|&c| expr.contains(c)).collect();
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum OptionBool {
    False,
    True,
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

impl From<bool> for OptionBool {
    fn from(b: bool) -> Self {
        if b {
            OptionBool::True
        } else {
            OptionBool::False
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Row {
    values: Vec<OptionBool>,
    id: Vec<usize>,
}

impl From<&Row> for u32 {
    fn from(row: &Row) -> Self {
        row.values.iter().rev().fold(0, |acc, x| {
            (acc << 1)
                | match x {
                    OptionBool::True => 1,
                    _ => 0,
                }
        })
    }
}

impl Row {
    fn new(id: usize, width: usize) -> Row {
        let mut values = vec![OptionBool::False; width];
        (0..width).for_each(|i| {
            values[i] = OptionBool::from((id >> (width - i - 1)) & 1 == 1);
        });
        Row {
            values,
            id: vec![id],
        }
    }

    /// get a bitfield for the care bits
    fn care(&self) -> u32 {
        let mut res = 0;
        for (i, v) in self.values.iter().enumerate() {
            if *v != OptionBool::DontCare {
                res |= 1 << (self.values.len() - i - 1);
            }
        }
        res
    }

    /// get the bit difference between two rows
    fn diff(&self, other: &Row) -> u32 {
        u32::from(self) ^ u32::from(other)
    }

    /// mark the desired bits as dont care
    fn mark(&mut self, mask: u32) {
        for (i, v) in self.values.iter_mut().enumerate().rev() {
            if (mask >> i) & 1 == 1 {
                *v = OptionBool::DontCare;
            }
        }
    }

    /// merge two rows
    fn merge(&self, other: &Row) -> Row {
        let mut res = self.clone();
        res.mark(self.diff(other));
        res.id.extend_from_slice(&other.id);
        res
    }

    fn can_merge(&self, other: &Row) -> bool {
        self.care() == other.care() && self.diff(other).count_ones() == 1
    }
}

impl Tree {
    pub fn cnf(&self) -> Tree {
        // Using the Quine-McCluskey algorithm
        // https://en.wikipedia.org/wiki/Quine%E2%80%93McCluskey_algorithm
        // https://electronics.stackexchange.com/questions/520513/can-quine-mccluskey-method-be-used-for-product-of-sum-simplification

        // Step 1: generate truth table
        let expr = self.root.to_string();
        let var_list: Vec<char> = ('A'..='Z').filter(|&c| expr.contains(c)).collect();
        let table = get_table(&expr, &expr);
        let bit_width = (table.len() - 1).count_ones() as usize;
        // we only need to look at the zero rows
        let false_rows: Vec<Row> = table
            .iter()
            .enumerate()
            .filter(|(_, &b)| !b)
            .map(|(i, _)| Row::new(i, bit_width))
            .collect();
        if false_rows.is_empty() || false_rows.len() == 1 << var_list.len() {
            // all true or all false
            return Tree {
                root: Node {
                    not: 0,
                    literal: Const(false_rows.is_empty()),
                },
                variables: self.variables.clone(),
            };
        }
        // Step 2: generate prime implicants by combining rows
        let mut prime_implicants = Vec::new();
        let mut done = false;
        let mut implicants = false_rows.clone();
        while !done {
            done = true;
            let mut new_implicants = Vec::new();
            let mut used = vec![false; implicants.len()];
            for i in 0..implicants.len() {
                let mut found = false;
                for j in i + 1..implicants.len() {
                    if implicants[i].can_merge(&implicants[j]) {
                        found = true;
                        used[j] = true;
                        // check if the new implicant is already in the list
                        let mut new_implicant = implicants[i].merge(&implicants[j]);
                        new_implicant.id.sort();
                        if !prime_implicants.contains(&new_implicant) {
                            new_implicants.push(new_implicant);
                        }
                    }
                }
                if found {
                    done = false;
                } else if !used[i] {
                    prime_implicants.push(implicants[i].clone());
                }
            }
            implicants = new_implicants;
        }
        prime_implicants.sort();
        prime_implicants.dedup();
        println!(
            "False rows: {:16}{:?}",
            "",
            false_rows.iter().map(|r| &r.id).collect::<Vec<_>>()
        );
        println!(
            "Prime implicants: {:10}{:?}",
            "",
            prime_implicants.iter().map(|r| &r.id).collect::<Vec<_>>()
        );
        // Step 3: generate essential prime implicants by checking if they cover all false rows
        // this is done by making sure that the id of every implicant is represented at least once
        let mut essential_prime_implicants = Vec::new();
        // the first step is to find the implicants that are the only ones that cover a row, if any
        let mut covered = vec![false; table.len()];
        for implicant in &false_rows {
            let mut count = 0;
            let mut index = 0;
            for (i, row) in prime_implicants.iter().enumerate() {
                if row.id.iter().any(|&id| id == implicant.id[0]) {
                    count += 1;
                    index = i;
                }
            }
            if count == 1 && !essential_prime_implicants.contains(&prime_implicants[index]) {
                essential_prime_implicants.push(prime_implicants[index].clone());
                for id in &prime_implicants[index].id {
                    covered[*id] = true;
                }
            }
        }
        println!("{:?}", covered);
        println!("{:?}", essential_prime_implicants);
        // now we need to find the best combination of implicants that cover all rows
        // this is done by implementing the Petrick's method
        // https://en.wikipedia.org/wiki/Petrick%27s_method
        let mut petrick = Vec::new();
        for implicant in &prime_implicants {
            // check if the implicant covers any row that is not covered yet
            if implicant.id.iter().any(|&id| !covered[id]) {
                petrick.push(implicant.id.clone());
            }
        }
        println!("{:?}", petrick);
        if !petrick.is_empty() {
            let pos: Vec<Vec<usize>> = covered
                .iter()
                .enumerate()
                .filter(|(_, &b)| !b)
                .map(|(i, _)| {
                    petrick
                        .iter()
                        .enumerate()
                        .filter(|(_, v)| v.iter().any(|&id| id == i))
                        .map(|(i, _)| i)
                        .collect()
                })
                .collect();
            println!("petrick: {:?}", pos);
            println!("petrick: {:?}", petrick);
            essential_prime_implicants = prime_implicants;
        }
        println!(
            "Essential prime implicants: {:?}",
            essential_prime_implicants
        );
        // let mut essential_prime_implicants = Vec::new();
        // let mut covered = vec![false; false_rows.len()];
        // for implicant in &prime_implicants {
        //     let mut found = false;
        //     for (i, row) in false_rows.iter().enumerate() {
        //         if covered[i] {
        //             continue;
        //         }
        //         let mut match_ = true;
        //         for (j, &bit) in implicant.iter().enumerate() {
        //             if bit != OptionBool::DontCare && bit != row[j] {
        //                 match_ = false;
        //                 break;
        //             }
        //         }
        //         if match_ {
        //             found = true;
        //             covered[i] = true;
        //         }
        //     }
        //     if found {
        //         essential_prime_implicants.push(implicant.clone());
        //     }
        // }
        let mut res: Vec<String> = Vec::new();
        for implicant in &essential_prime_implicants {
            let mut or_needed = 0;
            let mut s = String::new();
            for (j, bit) in implicant.values.iter().enumerate() {
                match bit {
                    // here we invert the bits because we're looking at the zero rows
                    OptionBool::False => {
                        s.push(var_list[j]);
                        or_needed += 1;
                    }
                    OptionBool::True => {
                        s.push(var_list[j]);
                        s.push('!');
                        or_needed += 1;
                    }
                    OptionBool::DontCare => {}
                }
            }
            for _ in 0..or_needed - 1 {
                s.push('|');
            }
            res.push(s);
        }
        res.sort();
        let mut res: String = res.concat();
        for _ in 0..essential_prime_implicants.len() - 1 {
            res.push('&');
        }
        res.parse().unwrap() // should never fail
    }
}

impl Node {
    fn not(&mut self) -> &mut Self {
        self.not = (self.not + 1) % 2;
        if self.not == 1 {
            if let Const(c) = &mut self.literal {
                *c = !*c;
                self.not = 0;
            }
        }
        self
    }

    pub fn simplify(self) -> Node {
        self
        // let mut new = self.clone();
        // new.not = self.not % 2;
        // match new.literal {
        //     Const(c) => Node {
        //         not: 0,
        //         literal: Const(c ^ (new.not == 1)),
        //     },
        //     Var(_) => new,
        //     Binary { op, children } => {
        //         let mut new_children = Vec::new();
        //         if op == Or || op == And {
        //             for child in &children {
        //                 if let Binary { op: o, children: c } = child.clone().simplify().literal {
        //                     if op == o {
        //                         new_children.extend(c);
        //                     } else {
        //                         new_children.push(child.clone().simplify());
        //                     }
        //                 } else {
        //                     new_children.push(child.clone().simplify());
        //                 }
        //             }
        //             let mut children = new_children;
        //             for i in 0..children.len() {
        //                 for j in (i + 1)..children.len() {
        //                     if children.get(j).is_none() {
        //                         continue;
        //                     }
        //                     if let NodeCmp::Equal = children[i].compare(&children[j]) {
        //                         children.remove(j);
        //                     }
        //                 }
        //             }
        //         }
        //         let mut new_children: Vec<Node> = Vec::new();
        //         match op {
        //             And => {
        //                 // iterate through children, while removing duplicates
        //                 // if any are false, return false
        //                 // if any are true, remove them
        //                 // if there are conflicting children, return false
        //                 for child in &children {
        //                     if let Const(c) = child.literal {
        //                         if c ^ (child.not == 1) {
        //                             continue;
        //                         }
        //                         return Node {
        //                             not: 0,
        //                             literal: Const(false),
        //                         };
        //                     }
        //                     let mut to_add = true;
        //                     for new_child in &new_children {
        //                         match child.compare(new_child) {
        //                             NodeCmp::Equal => {
        //                                 to_add = false;
        //                                 break;
        //                             }
        //                             NodeCmp::Opposite => {
        //                                 return Node {
        //                                     not: 0,
        //                                     literal: Const(false),
        //                                 };
        //                             }
        //                             NodeCmp::NotEqual => {}
        //                         }
        //                     }
        //                     if to_add {
        //                         new_children.push(child.clone());
        //                     }
        //                 }
        //                 match new_children.len() {
        //                     0 => Node {
        //                         not: 0,
        //                         literal: Const(true),
        //                     },
        //                     1 => new_children[0].clone(),
        //                     _ => Node {
        //                         not: 0,
        //                         literal: Binary {
        //                             op: And,
        //                             children: new_children,
        //                         },
        //                     },
        //                 }
        //             }
        //             Or => {
        //                 // iterate through children, while removing duplicates
        //                 // if any are true, return true
        //                 // if any are false, remove them
        //                 // if there are conflicting children, return true
        //                 for child in &children {
        //                     if let Const(c) = child.literal {
        //                         if c ^ (child.not == 1) {
        //                             return Node {
        //                                 not: 0,
        //                                 literal: Const(true),
        //                             };
        //                         }
        //                         continue;
        //                     }
        //                     let mut to_add = true;
        //                     for new_child in &new_children {
        //                         match child.compare(new_child) {
        //                             NodeCmp::Equal => {
        //                                 to_add = false;
        //                                 break;
        //                             }
        //                             NodeCmp::Opposite => {
        //                                 return Node {
        //                                     not: 0,
        //                                     literal: Const(true),
        //                                 };
        //                             }
        //                             NodeCmp::NotEqual => {}
        //                         }
        //                     }
        //                     if to_add {
        //                         new_children.push(child.clone());
        //                     }
        //                 }
        //                 match new_children.len() {
        //                     0 => Node {
        //                         not: 0,
        //                         literal: Const(false),
        //                     },
        //                     1 => new_children[0].clone(),
        //                     _ => Node {
        //                         not: 0,
        //                         literal: Binary {
        //                             op: Or,
        //                             children: new_children,
        //                         },
        //                     },
        //                 }
        //             }
        //             Xor => {
        //                 // Xor is not associative, so it's a bit different here
        //                 // it should only have two children
        //                 // if they are equal, return false
        //                 // if they are opposite, return true
        //                 // if one is true, return the other negated
        //                 // if one is false, return the other
        //                 // otherwise, return the xor of the two
        //                 if children.len() != 2 {
        //                     panic!("Xor should only have two children");
        //                 }
        //                 match children[0].compare(&children[1]) {
        //                     NodeCmp::Equal => Node {
        //                         not: 0,
        //                         literal: Const(false),
        //                     },
        //                     NodeCmp::Opposite => Node {
        //                         not: 0,
        //                         literal: Const(true),
        //                     },
        //                     NodeCmp::NotEqual => {
        //                         match (children[0].literal, children[1].literal) {
        //                             (Const(c), _) | (_, Const(c)) => {
        //                                 if c ^ (children[0].not == 1) {
        //                                     children[1].clone().not()
        //                                 } else {
        //                                     children[0].clone()
        //                                 }
        //                             }
        //                             _ => new,
        //                         };
        //                         if let Const(c) = children[0].literal {
        //                             let mut new = children[1].clone();
        //                             new.not().simplify()
        //                         } else if let Const(c) = children[1].literal {
        //                             let mut new = children[0].clone();
        //                             new.not().simplify()
        //                         } else {
        //                             self
        //                         }
        //                     }
        //                 }
        //             }
        //             Impl => {
        //                 // Impl is not associative, so it's a bit different here
        //                 // it should only have two children
        //                 // if the first is true, return the second
        //                 // if the first is false, return true
        //                 // if the second is true, return true
        //                 // if the second is false, return the first negated
        //                 // otherwise, return the impl of the two
        //                 if children.len() != 2 {
        //                     panic!("Impl should only have two children");
        //                 }
        //                 match children[0].compare(&children[1]) {
        //                     NodeCmp::Equal => Node {
        //                         not: 0,
        //                         literal: Const(true),
        //                     },
        //                     NodeCmp::Opposite => {
        //                         let mut new = children[0].clone();
        //                         new.not = (new.not + 1) % 2;
        //                         new.simplify()
        //                     }
        //                     NodeCmp::NotEqual => {
        //                         if let Const(c) = children[0].literal {
        //                             if c ^ (children[0].not == 1) {
        //                                 children[1].clone()
        //                             } else {
        //                                 Node {
        //                                     not: 0,
        //                                     literal: Const(true),
        //                                 }
        //                             }
        //                         } else if let Const(c) = children[1].literal {
        //                             if c ^ (children[1].not == 1) {
        //                                 Node {
        //                                     not: 0,
        //                                     literal: Const(true),
        //                                 }
        //                             } else {
        //                                 let mut new = children[0].clone();
        //                                 new.not = (new.not + 1) % 2;
        //                                 new.simplify()
        //                             }
        //                         } else {
        //                             self
        //                         }
        //                     }
        //                 }
        //             }
        //             Leq => {
        //                 // Leq is not associative, so it's a bit different here
        //                 // it should only have two children
        //                 // if they are equal, return true
        //                 // if they are opposite, return false
        //                 // if one is true, return the other
        //                 // if one is false, return the other negated
        //                 // otherwise, return the leq of the two
        //                 if children.len() != 2 {
        //                     panic!("Leq should only have two children");
        //                 }
        //                 match children[0].compare(&children[1]) {
        //                     NodeCmp::Equal => Node {
        //                         not: 0,
        //                         literal: Const(true),
        //                     },
        //                     NodeCmp::Opposite => Node {
        //                         not: 0,
        //                         literal: Const(false),
        //                     },
        //                     NodeCmp::NotEqual => {
        //                         if let Const(c) = children[0].literal {
        //                             if c ^ (children[0].not == 1) {
        //                             } else {
        //                                 children[1].clone()
        //                             }
        //                         } else if let Const(c) = children[1].literal {
        //                             if c ^ (children[1].not == 1) {
        //                                 children[0].clone()
        //                             } else {
        //                                 Node {
        //                                     not: 0,
        //                                     literal: Const(false),
        //                                 }
        //                             }
        //                         } else {
        //                             self
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    }
}
