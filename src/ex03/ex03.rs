// an AST to parse logical expressions in rpn

#[allow(dead_code)]
#[derive(Debug)]
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

// 101|&
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

fn main() {
    // lets try a long rpn expression
    let input = random_rpn_expr();
    println!("{}", input);
    let node = parse(&input).unwrap();
    // call draw_ast on the root node
    print_dot(&node);
    // for debugging purposes, print the AST in stderr
    eprintln!("{:?}", node);
}

use std::fs::File;
use std::io::Read;

fn random_rpn_expr() -> String {
    let mut rng = || {
        // get a random number from /dev/urandom
        let mut f = File::open("/dev/urandom").unwrap();
        let mut buf = [0u8; 1];
        f.read_exact(&mut buf).unwrap();
        buf[0] as usize
    };
    let mut expr = String::new();
    let mut stack = Vec::new();
    let mut binops = vec![
        '&', '|', '^', '>', '='
    ];
    for _ in 0..rng() % 100 {
        // add two values and a binary operator to the expression
        expr.push_str(&format!("{}", rng() % 2));
        expr.push_str(&format!("{}", rng() % 2));
        expr.push_str(&format!("{}", binops[rng() % binops.len()]));
    }
    expr
}

#[test]
fn tests() {
}
