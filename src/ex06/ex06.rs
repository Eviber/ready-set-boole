#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    fn test_cnf(formula: &str, expected: &str) {
        todo!();
    }

    #[test]
    fn ex06_basic_test() {
        test_cnf("AB&", "AB&");
        test_cnf("AB&!", "A!B!|");
        test_cnf("AB|!", "A!B!&");
        test_cnf("AB|C&", "AB|C&");
        // test_cnf("AB|C|D|", "ABCD|||");
        // test_cnf("AB&C&D&", "ABCD&&&");
        test_cnf("AB&!C!|", "A!B!|C!|");
        test_cnf("AB|!C!&", "A!B!&C!&");
    }

    #[test]
    fn ex06_random_test_cnf() {
        for _ in 0..1000 {
            let expr = random_rpn_expr(3, 5);
            let cnf = conjunctive_normal_form(&expr);
            assert_eq!(get_table(&cnf, &expr), get_table(&expr, &expr), "{}", expr);
        }
    }

    #[test]
    fn ex06_random_test_simplify() {
        for _ in 0..1000 {
            let expr = random_rpn_expr(3, 3);
            let simp = expr
                .parse::<Tree>()
                .expect("input is valid")
                .root
                .simplify()
                .to_string();
            assert_eq!(get_table(&simp, &expr), get_table(&expr, &expr), "{}", expr);
        }
    }
}
