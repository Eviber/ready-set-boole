use super::{get_table, Const, Node, Tree};
use crate::tree::row::{OptionBool, Row};

fn false_rows_from_table(table: &[bool], bit_width: usize) -> Vec<Row> {
    table
        .iter()
        .enumerate()
        .filter(|(_, &b)| !b)
        .map(|(i, _)| Row::new(i, bit_width))
        .collect()
}

fn prime_implicants_from_false_rows(false_rows: &[Row]) -> Vec<Row> {
    let mut done = false;
    let mut implicants: Vec<Row> = false_rows.to_vec();
    let mut prime_implicants = Vec::new();
    while !done {
        done = true;
        let mut new_implicants = Vec::new();
        let mut used = vec![false; implicants.len()];
        for i in 0..implicants.len() {
            let mut found = false;
            for j in i + 1..implicants.len() {
                if implicants[i].can_merge(&implicants[j]) {
                    found = true;
                    used[j] = true;
                    // check if the new implicant is already in the list
                    let mut new_implicant = implicants[i].merge(&implicants[j]);
                    new_implicant.id.sort_unstable();
                    if !prime_implicants.contains(&new_implicant) {
                        new_implicants.push(new_implicant);
                    }
                }
            }
            if found {
                done = false;
            } else if !used[i] {
                prime_implicants.push(implicants[i].clone());
            }
        }
        implicants = new_implicants;
    }
    prime_implicants.sort();
    prime_implicants.dedup();
    prime_implicants
}

fn essential_prime_implicants_from_prime_implicants(
    false_rows: &[Row],
    prime_implicants: Vec<Row>,
    rows_count: usize,
) -> Vec<Row> {
    let mut covered = vec![false; rows_count];
    let mut essential_prime_implicants = Vec::new();
    // the first step is to find the implicants that are the only ones that cover a row, if any
    for implicant in false_rows {
        let mut count = 0;
        let mut index = 0;
        for (i, row) in prime_implicants.iter().enumerate() {
            if row.id.iter().any(|&id| id == implicant.id[0]) {
                count += 1;
                index = i;
            }
        }
        if count == 1 && !essential_prime_implicants.contains(&prime_implicants[index]) {
            essential_prime_implicants.push(prime_implicants[index].clone());
            for id in &prime_implicants[index].id {
                covered[*id] = true;
            }
        }
    }
    // println!("{:?}", covered);
    // println!("{:?}", essential_prime_implicants);
    // now we need to find the best combination of implicants that cover all rows
    // this is done by implementing the Petrick's method
    // https://en.wikipedia.org/wiki/Petrick%27s_method
    let mut petrick = Vec::new();
    for implicant in &prime_implicants {
        // check if the implicant covers any row that is not covered yet
        if implicant.id.iter().any(|&id| !covered[id]) {
            petrick.push(implicant.id.clone());
        }
    }
    // println!("{:?}", petrick);
    if !petrick.is_empty() {
        let _pos: Vec<Vec<usize>> = covered
            .iter()
            .enumerate()
            .filter(|(_, &b)| !b)
            .map(|(i, _)| {
                petrick
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.iter().any(|&id| id == i))
                    .map(|(i, _)| i)
                    .collect()
            })
            .collect();
        // println!("petrick: {:?}", _pos);
        // println!("petrick: {:?}", petrick);
        essential_prime_implicants = prime_implicants;
    }
    essential_prime_implicants
}

impl Tree {
    pub fn cnf(&self) -> Tree {
        // Using the Quine-McCluskey algorithm
        // https://en.wikipedia.org/wiki/Quine%E2%80%93McCluskey_algorithm
        // https://electronics.stackexchange.com/questions/520513/can-quine-mccluskey-method-be-used-for-product-of-sum-simplification

        // Step 1: generate truth table
        let expr = self.root.to_string();
        let var_list: Vec<char> = ('A'..='Z').filter(|&c| expr.contains(c)).collect();
        let table = get_table(&expr, &expr);
        let bit_width = (table.len() - 1).count_ones() as usize;
        let false_rows = false_rows_from_table(&table, bit_width);
        if false_rows.is_empty() || false_rows.len() == 1 << var_list.len() {
            // all true or all false
            return Tree {
                root: Node {
                    not: 0,
                    literal: Const(false_rows.is_empty()),
                },
                variables: self.variables.clone(),
            };
        }
        // Step 2: generate prime implicants by combining rows
        let prime_implicants = prime_implicants_from_false_rows(&false_rows);
        println!(
            "False rows: {:16}{:?}",
            "",
            false_rows.iter().map(|r| &r.id).collect::<Vec<_>>()
        );
        println!(
            "Prime implicants: {:10}{:?}",
            "",
            prime_implicants.iter().map(|r| &r.id).collect::<Vec<_>>()
        );
        // Step 3: generate essential prime implicants by checking if they cover all false rows
        // this is done by making sure that the id of every implicant is represented at least once
        let essential_prime_implicants = essential_prime_implicants_from_prime_implicants(
            &false_rows,
            prime_implicants,
            table.len(),
        );
        let mut res: Vec<String> = Vec::new();
        for implicant in &essential_prime_implicants {
            let mut or_needed = 0;
            let mut s = String::new();
            for (j, bit) in implicant.values.iter().enumerate() {
                match bit {
                    // here we invert the bits because we're looking at the zero rows
                    OptionBool::False => {
                        s.push(var_list[j]);
                        or_needed += 1;
                    }
                    OptionBool::True => {
                        s.push(var_list[j]);
                        s.push('!');
                        or_needed += 1;
                    }
                    OptionBool::DontCare => {}
                }
            }
            for _ in 0..or_needed - 1 {
                s.push('|');
            }
            res.push(s);
        }
        res.sort();
        let mut res: String = res.concat();
        for _ in 0..essential_prime_implicants.len() - 1 {
            res.push('&');
        }
        res.parse().unwrap() // should never fail
    }
}
