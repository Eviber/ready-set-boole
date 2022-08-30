fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut result = vec![vec![]];
    for &elem in set {
        let mut new_result = Vec::new();
        for subset in result.iter_mut() {
            subset.push(elem); // add elem to each subset
            new_result.push(subset.clone()); // add subset to result
            subset.pop(); // remove elem from each subset
        }
        result.append(&mut new_result);
    }
    result.sort_by_key(|subset| subset.iter().len());
    result
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
    res.sort_by_key(|v| v.len());
    res
}

fn main() {
    let set = [1, 2, 3];
    let res = powerset(&set);
    println!("{:?}", res);
}
