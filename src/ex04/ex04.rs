use std::io::{stdout, Result, Write};

use ready_set_boole::BooleanExpression;

fn truth_table(w: &mut impl Write, expr: BooleanExpression) -> Result<()> {
    for variable in expr.variables() {
        write!(w, "| {} ", variable)?;
    }
    writeln!(w, "| = |")?;
    for _ in 0..=expr.variables_count() {
        write!(w, "|---")?;
    }
    writeln!(w, "|")?;
    for bitfield in 0..(1 << expr.variables_count()) {
        for idx in (0..expr.variables_count()).rev() {
            write!(w, "| {} ", bitfield >> idx & 1)?;
        }
        writeln!(w, "| {} |", expr.eval_vars(bitfield) as u8)?;
    }
    Ok(())
}

fn print_truth_table(formula: &str) {
    let expr = match formula.parse() {
        Ok(expr) => expr,
        Err(e) => return eprintln!("{e}"),
    };
    let _ = truth_table(&mut stdout().lock(), expr);
}

fn main() {
    let mut args = std::env::args().skip(1);
    if args.len() > 1 {
        eprintln!("Error: more than one argument");
        return;
    }
    if let Some(arg) = args.next() {
        print_truth_table(&arg);
    } else {
        print_truth_table("AB&C|");
    };
}

#[cfg(test)]
mod ex04 {
    use super::truth_table;

    fn test_truth_table(formula: &str, expected_truth_table: &str) {
        let mut s: Vec<u8> = Vec::new();
        truth_table(&mut s, formula.parse().unwrap()).unwrap();
        let s = str::from_utf8(&s).unwrap();
        assert_eq!(expected_truth_table, s);
    }

    #[test]
    fn subject_test() {
        test_truth_table(
            "AB&C|",
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
",
        );
    }

    #[test]
    fn single_var() {
        test_truth_table(
            "A",
            "\
| A | = |
|---|---|
| 0 | 0 |
| 1 | 1 |
",
        );
    }

    #[test]
    fn single_not() {
        test_truth_table(
            "A!",
            "\
| A | = |
|---|---|
| 0 | 1 |
| 1 | 0 |
",
        );
    }

    #[test]
    fn and() {
        test_truth_table(
            "AB&",
            "\
| A | B | = |
|---|---|---|
| 0 | 0 | 0 |
| 0 | 1 | 0 |
| 1 | 0 | 0 |
| 1 | 1 | 1 |
",
        );
    }

    #[test]
    fn or() {
        test_truth_table(
            "AB|",
            "\
| A | B | = |
|---|---|---|
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 1 |
",
        );
    }

    #[test]
    fn xor() {
        test_truth_table(
            "AB^",
            "\
| A | B | = |
|---|---|---|
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |
",
        );
    }

    #[test]
    fn imply() {
        test_truth_table(
            "AB>",
            "\
| A | B | = |
|---|---|---|
| 0 | 0 | 1 |
| 0 | 1 | 1 |
| 1 | 0 | 0 |
| 1 | 1 | 1 |
",
        );
    }

    #[test]
    fn eq() {
        test_truth_table(
            "AB=",
            "\
| A | B | = |
|---|---|---|
| 0 | 0 | 1 |
| 0 | 1 | 0 |
| 1 | 0 | 0 |
| 1 | 1 | 1 |
",
        );
    }
}
