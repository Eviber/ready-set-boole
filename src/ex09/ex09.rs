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
    sets: Vec<Vec<i32>>,
    dot: bool,
}

fn eval_set(formula: &str, sets: &[Vec<i32>]) -> Vec<i32> {
    match formula.parse::<Tree>() {
        Ok(tree) => tree.eval_set(sets),
        Err(e) => {
            eprintln!("{:?}", e);
            vec![]
        }
    }
}

fn parse_args() -> Result<Args, String> {
    let mut args = args();
    let mut expr = String::new();
    let mut sets = Vec::new();
    let mut dot = false;
    let path = args.next().unwrap_or_else(|| "ex09".to_string());

    for arg in args {
        if let Some(arg) = arg.strip_prefix('-') {
            for c in arg.chars() {
                match c {
                    'd' => dot = true,
                    'r' => {
                        if expr.is_empty() {
                            expr = random_rpn_expr(3, 5);
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
            let set: Result<Vec<i32>, _> = arg.split(',').map(str::parse).collect();
            match set {
                Ok(set) => sets.push(set),
                Err(_) => return Err(path),
            }
        }
    }
    if expr.is_empty() {
        Err(path)
    } else {
        Ok(Args { expr, sets, dot })
    }
}

fn main() -> Result<(), ParseError> {
    let (expr, sets, dot) = match parse_args() {
        Ok(args) => (args.expr, args.sets, args.dot),
        Err(path) => {
            println!("Usage: {} <formula sets | -r> [-d]", path);
            println!("formula: a propositional boolean formula in rpn, ex: AB&C|");
            println!("sets: a list of sets of integers, ex: 1,2,3 4,5,6");
            println!("Options:");
            println!("  -r  use a randomly generated formula");
            println!("  -d  print the dot graph of the formula and generate an image from it");
            return Ok(());
        }
    };
    println!("Input:\n{}", expr);
    if dot {
        create_graph(&expr.parse::<Tree>()?.root, "ex09_in");
    }
    println!("Sets:\n{:?}", sets);
    println!("{:?}", eval_set(&expr, &sets));
    Ok(())
}

#[cfg(test)]
mod tests {}
