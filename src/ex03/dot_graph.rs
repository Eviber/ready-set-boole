// prints a dot graph of the AST
// use dot -Tpng -o ex03.png ex03.dot

use crate::node::Node;
use crate::node::Node::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn create_graph(node: &Node) {
    let mut file = match File::create("ex03.dot") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating dot file: {}", e);
            return;
        }
    };
    let mut dot = String::new();
    let mut idx = HashMap::new();
    dot.push_str("digraph {\n");
    dot.push_str("\tnode [shape=none];\n");
    dot.push_str("\tedge [arrowhead=none];\n");
    dot.push('\n');
    print_dot_node(&mut dot, node, &mut idx);
    dot.push('}');
    match file.write_all(dot.as_bytes()) {
        Ok(_) => println!("Created dot file ex03.dot"),
        Err(e) => {
            eprintln!("Error writing to dot file: {}", e);
            return;
        }
    }
    match Command::new("dot")
        .args("-Tpng", "-o", "ex03.png", "ex03.dot")
        .output()
    {
        Ok(_) => println!("Created ex03.png"),
        Err(e) => eprintln!("Error running dot: {}, image may not be created", e),
    }
}

fn get_idx(node: &Node, idx: &mut HashMap<char, usize>) -> String {
    let mut get_id = |c: char| {
        let id = idx.entry(c).or_insert(0);
        // convert to a base-52 string
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
