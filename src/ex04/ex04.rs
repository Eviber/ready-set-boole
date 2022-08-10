// an AST to parse logical expressions in rpn

mod dot_graph;
mod expr_generator;
mod node;

use crate::node::Tree;
use dot_graph::create_graph;
use expr_generator::random_rpn_expr;
use node::ParseError;
use std::env::args;

fn eval_formula(formula: &str) -> bool {
    formula.parse::<Tree>().unwrap().root.into()
}

struct Args {
    expr: String,
    dot: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut args = args();
    let mut expr = String::new();
    let mut dot = false;
    let path = args.next().unwrap_or_else(|| "ex03".to_string());
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
    println!("{}", eval_formula(&expr));
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
