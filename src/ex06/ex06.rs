// an AST to parse logical expressions in rpn

mod dot_graph;
mod expr_generator;
mod node;

use crate::node::Tree;
use dot_graph::create_graph;
use expr_generator::random_rpn_expr;
use node::ParseError;
use std::env::args;

struct Args {
    expr: String,
    dot: bool,
}

fn conjunctive_normal_form(formula: &str) -> String {
    match formula.parse::<Tree>() {
        Ok(tree) => tree.root.cnf().simplify().to_string(),
        Err(e) => format!("Error: {:?}", e),
    }
}

fn parse_args() -> Result<Args, String> {
    let mut args = args();
    let mut expr = String::new();
    let mut dot = false;
    let path = args.next().unwrap_or_else(|| "ex06".to_string());

    for arg in args {
        if let Some(arg) = arg.strip_prefix('-') {
            for c in arg.chars() {
                match c {
                    'd' => dot = true,
                    'r' => {
                        if expr.is_empty() {
                            expr = random_rpn_expr(3, 5);
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
    let tree = expr.parse::<Tree>()?.root;
    if dot {
        create_graph(&tree, "ex06_in");
        create_graph(&(tree.cnf().simplify()), "ex06_out");
    }
    println!("{}", conjunctive_normal_form(&expr));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::BinOp::*;
    use crate::node::Node;
    use crate::node::{Literal, Variable};
    use crate::tests::Literal::{Binary, Const, Var};

    #[allow(dead_code)]
    fn test_cnf(formula: &str, expected: &str) {
        let cnf = conjunctive_normal_form(formula);
        assert_eq!(cnf, expected, "formula: {}", formula);
    }

    #[allow(dead_code)]
    fn get_table(input: &str, vars: &str) -> Vec<bool> {
        let tree = input.parse::<Tree>().expect("input is valid");
        let var_list: Vec<char> = ('A'..='Z').filter(|&c| vars.contains(c)).collect();
        let mut res = Vec::with_capacity(1 << var_list.len());
        for i in 0..(1 << var_list.len()) {
            for (j, v) in var_list.iter().enumerate() {
                let j = var_list.len() - j - 1;
                let bit = (i >> j) & 1;
                tree.set_var(*v, bit == 1);
            }
            res.push(tree.root.eval());
        }
        res
    }

    impl Tree {
        #[allow(dead_code)]
        fn set_var(&self, name: char, value: bool) {
            self.variables[name as usize - 'A' as usize].set(Variable { name, value });
        }
    }

    impl Node {
        #[allow(dead_code)]
        fn eval(&self) -> bool {
            let res = match &self.literal {
                Const(c) => *c,
                Var(v) => v.get().value,
                Binary { op, children } => {
                    let left = children[0].eval();
                    let right = children[1].eval();
                    match op {
                        And => left && right,
                        Or => left || right,
                        Impl => !left || right,
                        Leq => left == right,
                        Xor => left ^ right,
                    }
                }
            };
            res ^ (self.not % 2 == 1)
        }
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
