// an AST to parse logical expressions in rpn

#[derive(Debug, Clone, Copy)]
enum BinOp {
    And,
    Or,
    Xor,
    Impl,
    Leq,
}

#[derive(Debug)]
enum Node {
    Binary { op: BinOp, left: Box<Node>, right: Box<Node> },
    Not { operand: Box<Node> },
    Val(bool),
}

// error type
enum ParseError {
    MissingOperand,
    InvalidCharacter(char),
    UnbalancedExpression,
}

use Node::*;
use BinOp::*;
use ParseError::*;
use std::fmt;

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
                    stack.push(Not { operand: Box::new(operand) });
                }
                _ => {
                    let op = c.try_into()?; // convert char to BinOp
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

impl From<&Box<Node>> for bool {
    fn from(node: &Box<Node>) -> Self {
        Self::from(node.as_ref())
    }
}

impl From<&Node> for bool {
    fn from(node: &Node) -> Self {
        match node {
            Val(x) => *x,
            Not { operand } => !Self::from(operand),
            Binary { op, left, right } => {
                let left = || Self::from(left);
                let right = || Self::from(right);
                match op {
                    And =>  left() && right(),
                    Or => left() || right(),
                    Xor => left() ^ right(),
                    Impl => !left() || right(),
                    Leq => left() == right(),
                }
            },
        }
    }
}

impl From<Node> for bool {
    fn from(node: Node) -> Self {
        Self::from(&node)
    }
}

// prints a dot graph of the AST
// use dot -Tpng -o ex03.png ex03.dot

fn print_dot(node: &Node) {
    let mut dot = String::new();
    dot.push_str("digraph {\n");
    dot.push_str("node [shape=none];\n");
    dot.push_str("edge [arrowhead=none];\n");
    dot.push_str("\n");
    print_dot_node(&mut dot, node);
    dot.push_str("}");
    println!("{}", dot);
}

fn print_dot_node(dot: &mut String, node: &Node) {
    let id = get_node_addr(node);
    match node {
        Val(v) => {
            dot.push_str(&format!("{} [label=\"{}\"];\n", id, *v as u8));
        },
        Binary { op, left, right } => {
            dot.push_str(&format!("{} [label=\"{}\"];\n", id, op));
            dot.push_str(&format!("{} -> {};\n", id, get_node_addr(left)));
            dot.push_str(&format!("{} -> {};\n", id, get_node_addr(right)));
            print_dot_node(dot, left);
            print_dot_node(dot, right);
        }
        Not { operand } => {
            dot.push_str(&format!("{} [label=\"!\"];\n", id));
            dot.push_str(&format!("{} -> {};\n", id, get_node_addr(operand)));
            print_dot_node(dot, operand);
        }
    }
}

fn get_node_addr(node: &Node) -> usize {
    unsafe { std::mem::transmute(node) }
}

fn main() -> Result<(), ParseError> {
    let input = random_rpn_expr();
    eprintln!("Input:\n{}", input);
    let node = input.parse()?;
    eprintln!("{}", input);
    print_dot(&node);
    eprintln!("{}", bool::from(&node));
    Ok(())
}

use std::fs::File;
use std::io::Read;

fn random_rpn_expr() -> String {
    let rng = || {
        // get a random number from /dev/urandom
        let mut f = File::open("/dev/urandom").unwrap();
        let mut buf = [0u8; 1];
        f.read_exact(&mut buf).unwrap();
        buf[0] as usize
    };
    let mut rpn = String::new();
    let ops = vec![
        '&', '|', '^', '>', '=', '!', '0', '1',
    ];
    let vals = vec!['0', '1'];
    let mut needed = 1;
    while needed > 0 {
        // if the expression is too long, only use operators
        let op = if rpn.len() == 0 {
            ops[rng() % (ops.len() - 2)]
        } else {
            match needed {
                1..=3 => ops[rng() % ops.len()],
                _ => vals[rng() % vals.len()],
            }
        };
        // push the operator at the start of the expression
        rpn.insert(0, op);
        needed -= 1;
        needed += match op {
            '0' | '1' => 0,
            '!' => 1,
            _ => 2,
        };
    }
    rpn
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    fn to_bool(s: &str) -> bool {
        s.parse::<Node>().unwrap().into()
    }

    #[test]
    fn ex03_basic_tests() {
        assert_eq!(to_bool("0"), false);
        assert_eq!(to_bool("1"), true);
        assert_eq!(to_bool("1!"), false);
        assert_eq!(to_bool("0!"), true);
        assert_eq!(to_bool("11&"), true);
        assert_eq!(to_bool("10&"), false);
        assert_eq!(to_bool("10|"), true);
        assert_eq!(to_bool("01|"), true);
        assert_eq!(to_bool("00|"), false);
        assert_eq!(to_bool("10^"), true);
        assert_eq!(to_bool("11^"), false);
        assert_eq!(to_bool("10>"), false);
        assert_eq!(to_bool("01>"), true);
        assert_eq!(to_bool("10="), false);
        assert_eq!(to_bool("11="), true);
    }

    #[test]
    fn ex03_subject_tests() {
        assert_eq!(to_bool("10&"), false);
        assert_eq!(to_bool("10|"), true);
        assert_eq!(to_bool("11>"), true);
        assert_eq!(to_bool("10="), false);
        assert_eq!(to_bool("1011||="), true);
    }

    #[test]
    fn ex03_advanced_tests() {
        assert_eq!(to_bool("1011||="), true);
        assert_eq!(to_bool("111&!!!1|01=|=11>^0|0!1^11>1|0>1^>10^1|>10^>^"), true);
    }

    /*
    #[test]
    fn ex03_error_tests() {
        unwrap_err!("1&".parse::<Node>());
        unwrap_err!("1|".parse::<Node>());
        unwrap_err!("1>".parse::<Node>());
        unwrap_err!("1=".parse::<Node>());
        unwrap_err!("1^".parse::<Node>());
        unwrap_err!("1&1".parse::<Node>());
        unwrap_err!("1|1".parse::<Node>());
        unwrap_err!("1=1".parse::<Node>());
        unwrap_err!("1^1".parse::<Node>());
        unwrap_err!("1>1".parse::<Node>());

        unwrap_err!("1x|".parse::<Node>());
        unwrap_err!("1x&".parse::<Node>());
        unwrap_err!("1x>".parse::<Node>());
        unwrap_err!("1x=".parse::<Node>());
        unwrap_err!("1x^".parse::<Node>());
        unwrap_err!("1x!".parse::<Node>());

        unwrap_err!("10&1".parse::<Node>());
        unwrap_err!("10|1".parse::<Node>());
        unwrap_err!("10>1".parse::<Node>());
        unwrap_err!("10=1".parse::<Node>());
        unwrap_err!("10^1".parse::<Node>());
        unwrap_err!("10!".parse::<Node>());
    }
    */
}
