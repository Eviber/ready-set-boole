use crate::node::*;
use std::cell::Cell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

fn rng() -> usize {
    // get a random number from /dev/urandom
    let mut f = File::open("/dev/urandom").unwrap();
    let mut buf = [0u8; 1];
    f.read_exact(&mut buf).unwrap();
    buf[0] as usize
}

pub fn random_rpn_expr(maxdepth: u32, maxvars: usize) -> String {
    assert!(maxdepth > 0, "maxdepth must be > 0");
    let vals = (b'A'..=b'A' + (rng() % maxvars) as u8)
        .map(|x| x as char)
        .map(|x| {
            Rc::new(Cell::new(Variable {
                name: x,
                value: false,
            }))
        })
        .collect::<Vec<_>>();
    random_node(&vals, maxdepth).to_string()
}

fn random_node(vals: &[VarCell], maxdepth: u32) -> Node {
    use BinOp::*;
    use Node::*;

    if maxdepth == 0 {
        return Var(vals[rng() % vals.len()].clone());
    }
    let n = if maxdepth >= 5 {
        rng() % 6 + 1
    } else {
        rng() % 7
    };
    match n {
        0 => Var(vals[rng() % vals.len()].clone()),
        1 => Not(Box::new(random_node(vals, maxdepth - 1))),
        n => Binary {
            op: match n {
                2 => And,
                3 => Or,
                4 => Xor,
                5 => Impl,
                _ => Leq,
            },
            left: Box::new(random_node(vals, maxdepth - 1)),
            right: Box::new(random_node(vals, maxdepth - 1)),
        },
    }
}
