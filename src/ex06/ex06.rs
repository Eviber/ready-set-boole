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

fn negation_normal_form(formula: &str) -> String {
    match formula.parse::<Tree>() {
        Ok(tree) => tree.root.nnf().to_string(),
        Err(e) => format!("Error: {:?}", e),
    }
}

fn parse_args() -> Result<Args, String> {
    let mut args = args();
    let mut expr = String::new();
    let mut dot = false;
    let path = args.next().unwrap_or_else(|| "ex06".to_string());

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
    let tree = expr.parse::<Tree>()?.root;
    if dot {
        create_graph(&tree, "ex06_in");
        create_graph(&(tree.cnf()), "ex06_out");
    }
    println!("{}", negation_normal_form(&expr));
    Ok(())
}
