use ready_set_boole::BooleanExpression;
use std::fmt::Write;

fn truth_table(formula: &str) -> String {
    let expr: BooleanExpression = formula
        .parse()
        .expect("formula must be a valid boolean expression");
    let mut res = String::new();
    let variables = expr.variables().to_vec();
    for &variable in &variables {
        let _ = write!(res, "| {} ", variable);
    }
    res.push_str("| = |\n");
    let _ = writeln!(res, "{}|", "|---".repeat(variables.len() + 1));
    for bitfield in 0..(2_u32.pow(variables.len() as u32)) {
        res.push('|');
        for idx in (0..variables.len()).rev() {
            let value = bitfield >> idx & 1;
            let _ = write!(res, " {} |", value);
        }
        let _ = writeln!(
            res,
            " {} |",
            expr.eval_vars(bitfield) as u8
        );
    }
    res
}

fn print_truth_table(formula: &str) {
    println!("{}", truth_table(formula));
}

fn main() {
    print_truth_table("AB&C|");
}

#[cfg(test)]
mod tests {
    use super::truth_table;

    #[test]
    fn subject_test() {
        assert_eq!(
            truth_table("AB&C|"),
            "\
| A | B | C | = |
|---|---|---|---|
| 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 |
| 0 | 1 | 0 | 0 |
| 0 | 1 | 1 | 1 |
| 1 | 0 | 0 | 0 |
| 1 | 0 | 1 | 1 |
| 1 | 1 | 0 | 1 |
| 1 | 1 | 1 | 1 |
"
        );
    }
}
