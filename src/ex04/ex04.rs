use std::io::{stdout, Write};

use ready_set_boole::BooleanExpression;

fn truth_table(w: &mut impl Write, expr: BooleanExpression) {
    for variable in expr.variables() {
        let _ = write!(w, "| {} ", variable);
    }
    let _ = writeln!(w, "| = |");
    for _ in 0..=expr.variables_count() {
        let _ = write!(w, "|---");
    }
    let _ = writeln!(w, "|");
    for bitfield in 0..(1 << expr.variables_count()) {
        for idx in (0..expr.variables_count()).rev() {
            let _ = write!(w, "| {} ", bitfield >> idx & 1);
        }
        let _ = writeln!(w, "| {} |", expr.eval_vars(bitfield) as u8);
    }
}

fn print_truth_table(formula: &str) {
    let expr = match formula.parse() {
        Ok(expr) => expr,
        Err(e) => return eprintln!("{e}"),
    };
    truth_table(&mut stdout().lock(), expr);
}

fn main() {
    print_truth_table("AB&C|");
}

#[cfg(test)]
mod tests {
    use super::truth_table;

    #[test]
    fn subject_test() {
        let mut s: Vec<u8> = Vec::new();
        truth_table(&mut s, "AB&C|".parse().unwrap());
        let s = str::from_utf8(&s).unwrap();
        assert_eq!(
            s,
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
