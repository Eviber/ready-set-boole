use std::env::args;

fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    (0..1 << set.len())
        .map(|mask| {
            set.iter()
                .enumerate()
                .filter(|(n, _)| mask & (1 << n) != 0)
                .map(|(_, x)| *x)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn _powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut res = Vec::with_capacity(1 << set.len());
    for mask in 0..(1 << set.len()) {
        let mut tmp = Vec::new();
        for (i, &n) in set.iter().enumerate() {
            if mask & (1 << i) != 0 {
                tmp.push(n);
            }
        }
        res.push(tmp);
    }
    res
}

fn main() {
    args().skip(1).for_each(|arg| {
        println!(
            "{:?}",
            powerset(
                &arg.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            )
        );
    });
}
