use ready_set_boole::BooleanExpression;
use std::fmt::Write;

fn truth_table(expr: BooleanExpression) -> String {
    let mut res = String::new();
    for variable in expr.variables() {
        let _ = write!(res, "| {} ", variable);
    }
    res.push_str("| = |\n");
    for _ in 0..=expr.variables_count() {
        let _ = write!(res, "|---");
    }
    let _ = writeln!(res, "|");
    for bitfield in 0..(1 << expr.variables_count()) {
        for idx in (0..expr.variables_count()).rev() {
            let _ = write!(res, "| {} ", bitfield >> idx & 1);
        }
        let _ = writeln!(res, "| {} |", expr.eval_vars(bitfield) as u8);
    }
    res
}

fn print_truth_table(formula: &str) {
    let expr = match formula.parse() {
        Ok(expr) => expr,
        Err(e) => return eprintln!("{e}"),
    };
    print!("{}", truth_table(expr));
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
            truth_table("AB&C|".parse().unwrap()),
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
