// an AST to parse logical expressions in rpn

#[allow(dead_code)]
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

use Node::*;
use BinOp::*;

fn parse(input: &str) -> Result<Node, String> {
    let mut stack = Vec::with_capacity(42);

    for c in input.chars() {
        match c {
            '0' => stack.push(Val(false)),
            '1' => stack.push(Val(true)),
            '!' => {
                let operand = stack.pop().unwrap();
                stack.push(Not { operand: Box::new(operand) });
            },
            '&' | '|' | '^' | '>' | '=' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Binary {
                    op: match c {
                        '&' => And,
                        '|' => Or,
                        '^' => Xor,
                        '>' => Impl,
                        '=' => Leq,
                        _ => unreachable!(),
                    },
                    left: Box::new(left),
                    right: Box::new(right),
                });
            },
            _ => return Err(format!("unexpected character: {}", c)),
        }
    }
    if stack.len() != 1 {
        Err(format!("unbalanced expression: {}", input))
    } else {
        Ok(stack.pop().unwrap())
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            And => write!(f, "&"),
            Or => write!(f, "|"),
            Xor => write!(f, "^"),
            Impl => write!(f, ">"),
            Leq => write!(f, "="),
        }
    }
}

fn eval_binary_ref(op: BinOp, left: &Node, right: &Node) -> bool {
    let left = eval_tree_ref(left);
    let right = eval_tree_ref(right);
    match op {
        And => left & right,
        Or => left | right,
        Xor => left ^ right,
        Impl => !(left & !right),
        Leq => left == right,
    }
}

fn eval_tree_ref(node: &Node) -> bool {
    match node {
        Val(x) => *x,
        Not { operand } => !eval_tree_ref(operand),
        Binary { op, left, right } => eval_binary_ref(*op, left, right),
    }
}

fn eval_binary(op: BinOp, left: Node, right: Node) -> bool {
    let left = eval_tree(left);
    let right = eval_tree(right);
    match op {
        And => left & right,
        Or => left | right,
        Xor => left ^ right,
        Impl => !left | right,
        Leq => left == right,
    }
}

fn eval_tree(node: Node) -> bool {
    match node {
        Val(x) => x,
        Not { operand } => !eval_tree(*operand),
        Binary { op, left, right } => eval_binary(op, *left, *right),
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
            dot.push_str(&format!("{} [label=\"{}\"];\n", id, match v {
                true => "1",
                false => "0",
            }));
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
    use std::mem::transmute;
    unsafe { transmute(node) }
}

/*
fn print_node(node: Node) {
    print!("(");
    print_node(left);
    print!("{}",
        match op {
            And => " && ",
            Or => " || ",
            Xor => " ^ ",
            Impl => " -> ",
            Leq => " <= ",
        }
}
*/

fn main() {
    // lets try a long rpn expression
    let input = random_rpn_expr();
    let node = parse(&input).unwrap();
    // call draw_ast on the root node
    print_dot(&node);
    // for debugging purposes, print the AST in stderr
    eprintln!("{}", input);
    eprintln!("{}", eval_tree_ref(&node));
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

#[test]
fn tests() {
}
