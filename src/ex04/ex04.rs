// an AST to parse logical expressions in rpn

mod dot_graph;
mod expr_generator;
mod node;

use crate::node::Tree;
use dot_graph::create_graph;
use expr_generator::random_rpn_expr;
use node::ParseError;
use std::env::args;

struct Args {
    expr: String,
    dot: bool,
}

fn print_truth_table(formula: &str) {
    let var_list: Vec<char> = ('A'..='Z').filter(|&c| formula.contains(c)).collect();
    let tree = match formula.parse::<Tree>() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return;
        }
    };

    println!(
        "{}| = |",
        var_list
            .iter()
            .map(|v| format!("| {} ", v))
            .collect::<String>()
    ); // | A | B | ... | Z | = |
    println!("{}|", ("|---").repeat(var_list.len() + 1)); // |---|---| ... |---|
    for i in 0..(1 << var_list.len()) {
        let mut row = String::new();
        for (j, v) in var_list.iter().enumerate() {
            let j = var_list.len() - j - 1;
            let bit = (i >> j) & 1;
            tree.variables[*v as usize - 'A' as usize]
                .borrow_mut()
                .value = bit != 0;
            row.push_str(&format!("| {} ", bit));
        }
        println!("{}| {} |", row, tree.root.eval() as u8);
    }
}

fn parse_args() -> Result<Args, String> {
    let mut args = args();
    let mut expr = String::new();
    let mut dot = false;
    let path = args.next().unwrap_or_else(|| "ex04".to_string());
    for arg in args {
        if let Some(arg) = arg.strip_prefix('-') {
            for c in arg.chars() {
                match c {
                    'd' => dot = true,
                    'r' => {
                        if expr.is_empty() {
                            expr = random_rpn_expr();
                        } else {
                            return Err(path);
                        }
                    }
                    _ => return Err(path),
                }
            }
        } else if expr.is_empty() {
            expr = arg;
        } else {
            return Err(path);
        }
    }
    if expr.is_empty() {
        Err(path)
    } else {
        Ok(Args { expr, dot })
    }
}

fn main() -> Result<(), ParseError> {
    let (expr, dot) = match parse_args() {
        Ok(args) => (args.expr, args.dot),
        Err(path) => {
            println!("Usage: {} <formula | -r> [-d]", path);
            println!("formula: a propositional boolean formula in rpn, ex: AB&C|");
            println!("Options:");
            println!("  -r  use a randomly generated formula");
            println!("  -d  print the dot graph of the formula and generate an image from it");
            return Ok(());
        }
    };
    println!("Input:\n{}", expr);
    let formula = expr.parse::<Tree>()?;
    if dot {
        create_graph(&formula.root);
    }
    print_truth_table(&expr);
    Ok(())
}
