use std::fs::File;
use std::io::Read;

pub fn random_rpn_expr() -> String {
    let rng = || {
        // get a random number from /dev/urandom
        let mut f = File::open("/dev/urandom").unwrap();
        let mut buf = [0u8; 1];
        f.read_exact(&mut buf).unwrap();
        buf[0] as usize
    };
    let mut rpn = String::new();
    let ops = vec![
        '&', '|', '^', '>', '=', '!', '0', '1',
    ];
    let vals = vec!['0', '1'];
    let mut needed = 1;
    while needed > 0 {
        // if the expression is too long, only use operators
        let op = if rpn.is_empty() {
            ops[rng() % (ops.len() - 2)]
        } else {
            match needed {
                1..=3 => ops[rng() % ops.len()],
                _ => vals[rng() % vals.len()],
            }
        };
        // push the operator at the start of the expression
        rpn.insert(0, op);
        needed -= 1;
        needed += match op {
            '0' | '1' => 0,
            '!' => 1,
            _ => 2,
        };
    }
    rpn
}
