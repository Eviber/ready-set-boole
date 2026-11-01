use ready_set_boole::BooleanExpression;
use std::fmt::Write;

fn truth_table(formula: &str) -> String {
    let variables: Vec<_> = ('A'..='Z').filter(|&c| formula.contains(c)).collect();
    if variables.is_empty() {
        let expr: BooleanExpression = formula
            .parse()
            .expect("formula must be a valid boolean expression");
        return format!("Always {:?}", expr.eval());
    }
    let mut res = String::new();
    for &variable in &variables {
        let _ = write!(res, "| {} ", variable);
    }
    res.push_str("| = |\n");
    let _ = writeln!(res, "{}|", "|---".repeat(variables.len() + 1));
    for bitfield in 0..(2_u32.pow(variables.len() as u32)) {
        let mut formula = formula.to_string();
        res.push('|');
        for (value, variable) in variables.iter().enumerate().map(|(i, &v)| {
            (
                if (bitfield >> ((variables.len() - i - 1) as u32) & 1) == 0 {
                    "0"
                } else {
                    "1"
                },
                v,
            )
        }) {
            formula = formula.replace(variable, value);
            let _ = write!(res, " {} |", value);
        }
        let _ = writeln!(
            res,
            " {} |",
            formula.parse::<BooleanExpression>().unwrap().eval() as u8
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
