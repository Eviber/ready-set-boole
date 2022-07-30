// prints a dot graph of the AST
// use dot -Tpng -o ex03.png ex03.dot

use crate::node::Node;
use crate::node::Node::*;
use std::collections::HashMap;

pub fn print_dot(node: &Node) {
    let mut dot = String::new();
    let mut idx = std::collections::HashMap::new();
    dot.push_str("digraph {\n");
    dot.push_str("\tnode [shape=none];\n");
    dot.push_str("\tedge [arrowhead=none];\n");
    dot.push('\n');
    print_dot_node(&mut dot, node, &mut idx);
    dot.push('}');
    println!("{}", dot);
}

fn get_idx(node: &Node, idx: &mut HashMap<char, usize>) -> String {
    let mut get_id = |c: char| {
        let id = idx.entry(c).or_insert(0);
        // now convert to a base-52 string
        let mut s = String::new();
        let mut n = *id;
        if n == 0 {
            s.push('A');
        }
        while n > 0 {
            let c = (n % 52) as u8;
            let c = if c < 26 {
                (b'A' + c) as char
            } else {
                (b'a' + c - 26) as char
            };
            s.push(c as char);
            n /= 52;
        }
        *id += 1;
        s
    };
    match node {
        Val(v) => {
            let v = if *v { '1' } else { '0' };
            let id = get_id(v);
            format!("\"{}_{}\"", v, id)
        }
        Not { .. } => {
            let id = get_id('!');
            format!("\"!_{}\"", id)
        }
        Binary { op, .. } => {
            let id = get_id((*op).into());
            format!("\"{}_{}\"", op, id)
        }
    }
}

fn print_dot_node(dot: &mut String, node: &Node, idx: &mut HashMap<char, usize>) -> String {
    let id = get_idx(node, idx);
    match node {
        Val(v) => {
            dot.push_str(&format!("\t{} [label=\"{}\"];\n", id, *v as u8));
        }
        Binary { op, left, right } => {
            dot.push_str(&format!("\t{} [label=\"{}\"];\n", id, op));
            let left_id = print_dot_node(dot, left, idx);
            dot.push_str(&format!("\t{} -> {};\n", id, left_id));
            let right_id = print_dot_node(dot, right, idx);
            dot.push_str(&format!("\t{} -> {};\n", id, right_id));
        }
        Not { operand } => {
            dot.push_str(&format!("\t{} [label=\"!\"];\n", id));
            let operand_id = print_dot_node(dot, operand, idx);
            dot.push_str(&format!("\t{} -> {};\n", id, operand_id));
        }
    }
    id
}
