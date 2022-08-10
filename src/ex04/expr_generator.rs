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
    let ops = vec!['&', '|', '^', '>', '=', '!'];
    let vals: Vec<char> = (b'A'..=b'A' + (rng() % 26) as u8)
        .map(|x| x as char)
        .collect();
    let mut needed = 1;
    while needed > 0 {
        let op = if rpn.is_empty() {
            ops[rng() % ops.len()]
        } else {
            if needed > 3 || rng() % 4 == 0 {
                vals[rng() % vals.len()]
            } else {
                ops[rng() % ops.len()]
            }
        };
        rpn.insert(0, op);
        needed -= 1;
        needed += match op {
            'A'..='Z' => 0,
            '!' => 1,
            _ => 2,
        };
    }
    rpn
}
