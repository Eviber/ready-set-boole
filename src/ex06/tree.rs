mod cnf;
pub mod node;
mod row;

use binop::ParseError;
use node::Literal::{Binary, Const, Var};
pub use node::*;
use std::cell::Cell;
use std::rc::Rc;
use ParseError::{MissingOperand, UnbalancedExpression};

pub struct Tree {
    pub root: Node,
    pub variables: Vec<VarCell>,
}

impl std::str::FromStr for Tree {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::with_capacity(s.len());
        let variables: Vec<VarCell> = ('A'..='Z')
            .map(|c| {
                Rc::new(Cell::new(Variable {
                    name: c,
                    value: false,
                }))
            })
            .collect();

        for c in s.chars() {
            match c {
                '0' | '1' => stack.push(Node {
                    not: 0,
                    literal: Const(c == '1'),
                }),
                'A'..='Z' => stack.push(Node {
                    not: 0,
                    literal: Var(variables[c as usize - b'A' as usize].clone()),
                }),
                '!' => {
                    let operand = stack.pop().ok_or(MissingOperand)?;
                    stack.push(Node {
                        not: operand.not + 1,
                        literal: operand.literal,
                    });
                }
                _ => {
                    let tmp = stack.pop().ok_or(MissingOperand)?; // for the reverse pop order
                    let literal = Binary {
                        op: BinOp::try_from(c)?,
                        children: vec![stack.pop().ok_or(MissingOperand)?, tmp],
                    };
                    stack.push(Node { not: 0, literal });
                }
            }
        }
        if stack.len() == 1 {
            Ok(Tree {
                root: stack.pop().unwrap(),
                variables,
            })
        } else {
            Err(UnbalancedExpression)
        }
    }
}

pub fn get_table(input: &str, expr: &str) -> Vec<bool> {
    let tree = input.parse::<Tree>().expect("input is valid");
    let var_list: Vec<char> = ('A'..='Z').filter(|&c| expr.contains(c)).collect();
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
    fn set_var(&self, name: char, value: bool) {
        self.variables[name as usize - 'A' as usize].set(Variable { name, value });
    }
}
