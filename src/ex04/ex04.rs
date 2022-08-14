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
    color: bool,
}

fn print_truth_table(formula: &str) {
    match print_truth_table_color(formula, false) {
        Ok(_) => (),
        Err(e) => eprintln!("{:?}", e),
    }
}

fn color_bit(bit: u32, color: bool) -> String {
    if !color {
        return format!("{}", bit);
    }
    match bit {
        0 => "\x1b[31m0\x1b[0m".to_string(),
        1 => "\x1b[32m1\x1b[0m".to_string(),
        _ => unreachable!(),
    }
}

fn blue(s: &str) -> String {
    format!("\x1b[1;34m{}\x1b[0m", s)
}

fn print_truth_table_color(formula: &str, color: bool) -> Result<(), ParseError> {
    use std::io::{BufWriter, Write};
    let tree = formula.parse::<Tree>()?;
    let var_list: Vec<char> = ('A'..='Z').filter(|&c| formula.contains(c)).collect();
    let out = std::io::stdout();
    let mut buf = BufWriter::new(out.lock());
    let bar = if color { blue("|") } else { "|".to_string() };

    writeln!(
        buf,
        "{}{} = |",
        var_list
            .iter()
            .map(|v| format!("| {} ", v))
            .collect::<String>(),
        bar
    )
    .unwrap(); // | A | B | ... | Z | = |
    writeln!(buf, "{}{}---|", ("|---").repeat(var_list.len()), bar).unwrap(); // |---|---| ... |---|
    for i in 0..(1u32 << var_list.len()) {
        for (j, v) in var_list.iter().enumerate() {
            let j = var_list.len() - j - 1;
            let bit = (i >> j) & 1;
            tree.variables[*v as usize - 'A' as usize]
                .borrow_mut()
                .value = bit != 0;
            write!(buf, "| {} ", color_bit(bit, color)).unwrap();
        }
        writeln!(
            buf,
            "{} {} |",
            bar,
            color_bit(tree.root.eval() as u32, color)
        )
        .unwrap();
    }
    Ok(())
}

fn parse_args() -> Result<Args, String> {
    let mut args = args();
    let mut expr = String::new();
    let mut dot = false;
    let mut color = false;
    let path = args.next().unwrap_or_else(|| "ex04".to_string());

    for arg in args {
        if let Some(arg) = arg.strip_prefix('-') {
            for c in arg.chars() {
                match c {
                    'd' => dot = true,
                    'c' => color = true,
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
        Ok(Args { expr, dot, color })
    }
}

fn main() -> Result<(), ParseError> {
    let (expr, dot, color) = match parse_args() {
        Ok(args) => (args.expr, args.dot, args.color),
        Err(path) => {
            println!("Usage: {} <formula | -r> [-c] [-d]", path);
            println!("formula: a propositional boolean formula in rpn, ex: AB&C|");
            println!("Options:");
            println!("  -r  use a randomly generated formula");
            println!("  -c  color the truth table");
            println!("  -d  print the dot graph of the formula and generate an image from it");
            return Ok(());
        }
    };
    println!("Input:\n{}", expr);
    let formula = expr.parse::<Tree>()?;
    if dot {
        create_graph(&formula.root);
    }
    if color {
        print_truth_table_color(&expr, color)?;
    } else {
        print_truth_table(&expr);
    }
    Ok(())
}
