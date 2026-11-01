use ready_set_boole::BooleanExpression;

fn eval_formula(formula: &str) -> bool {
    formula.parse::<BooleanExpression>().unwrap().eval()
}

fn print_eval_formula(formula: &str) {
    println!("{} -> {}", formula, eval_formula(formula));
}

fn main() {
    print_eval_formula("10&");
    // false
    print_eval_formula("10|");
    // true
    print_eval_formula("11>");
    // true
    print_eval_formula("10=");
    // false
    print_eval_formula("1011||=");
    // true
}

#[cfg(test)]
mod tests {
    use super::eval_formula;

    #[test]
    fn ex03_basic_tests() {
        assert!(!eval_formula("0"));
        assert!(eval_formula("1"));
        assert!(!eval_formula("1!"));
        assert!(eval_formula("0!"));
        assert!(eval_formula("11&"));
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("01|"));
        assert!(!eval_formula("00|"));
        assert!(eval_formula("10^"));
        assert!(!eval_formula("11^"));
        assert!(!eval_formula("10>"));
        assert!(eval_formula("01>"));
        assert!(!eval_formula("10="));
        assert!(eval_formula("11="));
    }

    #[test]
    fn ex03_subject_tests() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11>"));
        assert!(!eval_formula("10="));
        assert!(eval_formula("1011||="));
    }

    #[test]
    fn ex03_advanced_tests() {
        assert!(eval_formula("1011||="));
        assert!(eval_formula(
            "111&!!!1|01=|=11>^0|0!1^11>1|0>1^>10^1|>10^>^"
        ));
    }
}
