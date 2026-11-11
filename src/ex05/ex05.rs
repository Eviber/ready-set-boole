use ready_set_boole::BooleanExpression;

fn negation_normal_form(formula: &str) -> String {
    let expr: BooleanExpression = match formula.parse() {
        Ok(expr) => expr,
        Err(e) => panic!("{e}"),
    };
    todo!()
}

fn main() {
    println!("{}", negation_normal_form("AB&!"));
    // A!B!|
    println!("{}", negation_normal_form("AB|!"));
    // A!B!&
    println!("{}", negation_normal_form("AB>"));
    // A!B|
    println!("{}", negation_normal_form("AB="));
    // AB&A!B!&|
    println!("{}", negation_normal_form("AB|C&!"));
    // A!B!&C!|
}

#[cfg(test)]
mod ex05 {
    use super::negation_normal_form;

    #[test]
    fn subject_tests() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }
}
