// an AST to parse logical expressions in rpn

mod dot_graph;
mod expr_generator;
mod node;

use crate::node::Tree;
use dot_graph::create_graph;
use expr_generator::random_rpn_expr;
use node::ParseError;
use std::env::args;
use std::sync::mpsc;

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

fn color_bit(bit: bool, color: bool) -> String {
    if !color {
        return format!("{}", u8::from(bit));
    }
    if bit {
        "\x1b[32m1\x1b[0m".to_string()
    } else {
        "\x1b[31m0\x1b[0m".to_string()
    }
}

fn blue(s: &str) -> String {
    format!("\x1b[1;34m{}\x1b[0m", s)
}

fn line_size(rows: usize, color: bool, bar: &str) -> usize {
    let bit = color_bit(false, color).len();
    let bar = bar.len();
    (3 + bit) * rows + bar
}

fn line_from_bitfield(
    i: usize,
    line_size: usize,
    var_list: &[char],
    tree: &Tree,
    color: bool,
    bar: &str,
) -> String {
    use std::fmt::Write;
    let mut line = String::with_capacity(line_size);
    for (j, v) in var_list.iter().enumerate() {
        let j = var_list.len() - j - 1;
        let bit: bool = (i >> j) & 1 == 1;
        tree.variables[*v as usize - 'A' as usize]
            .borrow_mut()
            .value = bit;
        write!(line, "| {} ", color_bit(bit, color)).unwrap();
    }
    write!(line, "{} {} |", bar, color_bit(tree.root.eval(), color)).unwrap();
    line
}

struct TableData {
    vars: Vec<char>,
    bar: String,
    max_value: usize,
    threads: usize,
}

fn print_table_lines(channels: Vec<mpsc::Receiver<String>>, data: &TableData) {
    use std::io::{BufWriter, Write};
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    data.vars
        .iter()
        .for_each(|&c| write!(out, "| {} ", c).unwrap());
    writeln!(out, "{} = |", data.bar).unwrap(); // | A | B | ... | Z | = |
    data.vars.iter().for_each(|_| write!(out, "|---").unwrap());
    writeln!(out, "{}---|", data.bar).unwrap(); // |---|---| ... |---|
    for i in 0..data.max_value {
        let line = channels[i % data.threads].recv().unwrap();
        writeln!(out, "{}", line).unwrap();
    }
    drop(channels);
}

#[inline]
fn needed_threads(max_value: usize) -> usize {
    use std::cmp::min;
    use std::num::NonZeroUsize;
    use std::thread::available_parallelism;
    min(
        max_value,
        available_parallelism()
            .unwrap_or(NonZeroUsize::new(2).unwrap())
            .get()
            - 1,
    )
}

fn print_truth_table_color(formula: &str, color: bool) -> Result<(), ParseError> {
    use std::thread;
    drop(formula.parse::<Tree>()?); // check for parsing errors
    let vars: Vec<char> = ('A'..='Z').filter(|&c| formula.contains(c)).collect();
    let bar = if color { blue("|") } else { "|".to_string() };

    thread::scope(|s| {
        let max_value: usize = 1 << vars.len();
        let threads = needed_threads(max_value);
        let data = TableData {
            vars,
            bar,
            max_value,
            threads,
        };
        let mut channels = Vec::with_capacity(data.threads);
        let line_size = line_size(data.vars.len(), color, &data.bar);

        for offset in 0..data.threads {
            let (tx, rx) = mpsc::sync_channel(16);
            channels.push(rx);
            let vars_cpy = data.vars.clone();
            let bar = data.bar.clone();
            s.spawn(move || {
                let tree = formula.parse::<Tree>().unwrap();
                for i in (offset..(data.max_value)).step_by(data.threads) {
                    let line = line_from_bitfield(i, line_size, &vars_cpy, &tree, color, &bar);
                    tx.send(line).unwrap();
                }
            });
        }
        print_table_lines(channels, &data);
    });
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
