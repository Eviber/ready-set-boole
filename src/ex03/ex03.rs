// an AST to parse logical expressions in rpn

mod dot_graph;
mod expr_generator;
mod node;

use dot_graph::print_dot;
use expr_generator::random_rpn_expr;
use node::ParseError;

fn main() -> Result<(), ParseError> {
    let input = random_rpn_expr();
    eprintln!("Input:\n{}", input);
    let node = input.parse()?;
    print_dot(&node);
    eprintln!("{}", bool::from(node));
    Ok(())
}

// tests
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
