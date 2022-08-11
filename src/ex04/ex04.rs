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

#[cfg(test)]
mod tests {
    use crate::node::Node;

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
        assert_eq!(
            to_bool("111&!!!1|01=|=11>^0|0!1^11>1|0>1^>10^1|>10^>^"),
            true
        );
    }

    #[test]
    fn ex03_error_tests() {
        use super::ParseError::*;
        assert_eq!("1&".parse::<Node>().err(), Some(MissingOperand));
        assert_eq!("1|".parse::<Node>().err(), Some(MissingOperand));
        assert_eq!("1>".parse::<Node>().err(), Some(MissingOperand));
        assert_eq!("1=".parse::<Node>().err(), Some(MissingOperand));
        assert_eq!("1^".parse::<Node>().err(), Some(MissingOperand));

        assert_eq!("00&1".parse::<Node>().err(), Some(UnbalancedExpression));
        assert_eq!("01|1".parse::<Node>().err(), Some(UnbalancedExpression));
        assert_eq!("10=1".parse::<Node>().err(), Some(UnbalancedExpression));
        assert_eq!("11^1".parse::<Node>().err(), Some(UnbalancedExpression));
        assert_eq!("00>1".parse::<Node>().err(), Some(UnbalancedExpression));

        assert_eq!("1x|".parse::<Node>().err(), Some(InvalidCharacter('x')));
        assert_eq!("1x&".parse::<Node>().err(), Some(InvalidCharacter('x')));
        assert_eq!("1x>".parse::<Node>().err(), Some(InvalidCharacter('x')));
        assert_eq!("1x=".parse::<Node>().err(), Some(InvalidCharacter('x')));
        assert_eq!("1x^".parse::<Node>().err(), Some(InvalidCharacter('x')));
        assert_eq!("1x!".parse::<Node>().err(), Some(InvalidCharacter('x')));
    }
}
