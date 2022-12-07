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

fn display_row(row: &Row, bit_width: usize) -> String {
    let mut s = String::new();
    for _ in 0..bit_width {
        for b in &row.values {
            match b {
                OptionBool::True => s.push('1'),
                OptionBool::False => s.push('0'),
                OptionBool::DontCare => s.push('-'),
            }
        }
    }
    s
}

fn display_rows(rows: &[Row], bit_width: usize) -> String {
    let mut s = String::new();
    for row in rows {
        s.push_str(&display_row(row, bit_width));
        s.push('\n');
    }
    s
}

// TODO: rewrite this function
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
                    // check if the new implicant is already in the list
                    let mut new_implicant = implicants[i].merge(&implicants[j]);
                    new_implicant.id.sort_unstable();
                    if !prime_implicants.contains(&new_implicant) {
                        new_implicants.push(new_implicant);
                        found = true;
                        used[j] = true;
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
        implicants.sort_unstable();
        implicants.dedup();
    }
    prime_implicants.sort();
    prime_implicants.dedup();
    prime_implicants
}

// type Product = Vec<usize>;
// type Sum = Vec<usize>;
// type ProductOfSums = Vec<Sum>;
// type SumOfProducts = Vec<Product>;
// type ProductOfSOPs = Vec<SumOfProducts>;

// lets use structs instead
// also generics

use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Var(usize);

impl Deref for Var {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Var {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<usize> for Var {
    fn from(id: usize) -> Self {
        Var(id)
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        fn usize_to_char(n: usize) -> char {
            let v = n as u8;
            (if v > 25 { v + b'A' - 26 } else { v + b'a' }) as char
        }
        write!(f, "{}", usize_to_char(self.0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Product<T> {
    factors: Vec<T>,
}

impl<T> Deref for Product<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.factors
    }
}

impl<T> DerefMut for Product<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.factors
    }
}

impl<T> Display for Product<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.factors.len() > 1 {
            write!(f, "(")?;
        }
        for factor in &self.factors {
            write!(f, "{}", factor)?;
        }
        if self.factors.len() > 1 {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl<T> From<Vec<T>> for Product<T> {
    fn from(v: Vec<T>) -> Self {
        Self { factors: v }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Sum<T> {
    terms: Vec<T>,
}

impl<T> Deref for Sum<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.terms
    }
}

impl<T> DerefMut for Sum<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terms
    }
}

impl<T> Display for Sum<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.terms.len() > 1 {
            write!(f, "(")?;
        }
        let mut first = true;
        for term in &self.terms {
            if first {
                first = false;
            } else {
                write!(f, "+")?;
            }
            write!(f, "{}", term)?;
        }
        if self.terms.len() > 1 {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl<T> From<Vec<T>> for Sum<T> {
    fn from(v: Vec<T>) -> Self {
        Self { terms: v }
    }
}

#[inline]
fn subset_of<T>(a: &[T], b: &[T]) -> bool
where
    T: PartialEq,
{
    a.len() < b.len() && a.iter().all(|x| b.contains(x))
}

impl<T> Sum<T>
where
    T: PartialEq,
{
    fn contains(&self, other: &Self) -> bool {
        subset_of(&other.terms, &self.terms)
    }
}

impl<T> Product<T>
where
    T: PartialEq,
{
    fn contains(&self, other: &Self) -> bool {
        subset_of(&other.factors, &self.factors)
    }
}

impl<T> FromIterator<T> for Product<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            factors: iter.into_iter().collect(),
        }
    }
}

impl<T> FromIterator<T> for Sum<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            terms: iter.into_iter().collect(),
        }
    }
}

impl<T> IntoIterator for Product<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.factors.into_iter()
    }
}

impl<T> IntoIterator for Sum<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.terms.into_iter()
    }
}

fn petricks_method(mut prime_implicants: Vec<Row>, covered: Vec<bool>) -> Vec<Row> {
    // now we need to find the best combination of implicants that cover all rows
    // this is done by implementing the Petrick's method
    // https://en.wikipedia.org/wiki/Petrick%27s_method
    prime_implicants.retain(|r| r.id.iter().any(|&i| !covered[i]));
    println!(
        "prime_implicants: {:?}",
        prime_implicants
            .iter()
            .map(|r| r.id.clone())
            .collect::<Vec<_>>()
    );
    if prime_implicants.is_empty() {
        return prime_implicants;
    }
    let mut product: Product<Sum<Var>> = (0..covered.len())
        .filter(|&i| !covered[i])
        .filter_map(|i| {
            let mut sum: Sum<Var> = prime_implicants
                .iter()
                .enumerate()
                .filter(|(_, r)| r.id.contains(&i))
                .map(|(j, _)| Var::from(j))
                .collect::<Vec<_>>()
                .into();
            sum.sort_unstable();
            if sum.is_empty() {
                None
            } else {
                Some(sum)
            }
        })
        .collect::<Vec<_>>()
        .into();
    println!("product: {}", product);
    product.sort_by(|a, b| {
        if a.len() == b.len() {
            a[0].cmp(&b[0])
        } else {
            a.len().cmp(&b.len())
        }
    });
    product.dedup();
    println!("product: {}", product);
    // now we distribute the product
    let mut sum = distribute(product);
    println!("sum: {}", sum);
    println!("SUM ^");
    // now we need to find the smallest sum
    let min = sum.iter().map(|v| v.len()).min().unwrap();
    println!("min: {}", min);
    sum.retain(|v| v.len() == min);
    println!("sum: {}", sum);
    println!("MIN ^");
    // and now find terms with fewest literals
    let min = sum
        .iter()
        .map(|v| {
            v.iter()
                .map(|r| {
                    prime_implicants[**r]
                        .values
                        .iter()
                        .map(|&b| match b {
                            OptionBool::True => 1,
                            OptionBool::False => 2,
                            OptionBool::DontCare => 0,
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .enumerate()
        .min_by_key(|(_, v)| *v)
        .map(|(i, _)| i)
        .unwrap();
    sum[min]
        .iter()
        .map(|&i| prime_implicants[*i].clone())
        .collect()
}

#[inline]
fn difference<T>(a: &[T], b: &[T]) -> usize
where
    T: PartialEq,
{
    a.iter().filter(|&x| !b.contains(x)).count() + b.iter().filter(|&x| !a.contains(x)).count()
}

impl<T> Product<T>
where
    T: PartialEq,
{
    fn difference(&self, other: &Self) -> usize {
        difference(&self.factors, &other.factors)
    }
}

impl<T> Sum<T>
where
    T: PartialEq,
{
    fn difference(&self, other: &Self) -> usize {
        difference(&self.terms, &other.terms)
    }
}

// TODO: merge theses

impl<T> Mul for Sum<T>
where
    T: std::cmp::PartialEq + std::clone::Clone + Mul<Output = Product<T>>,
{
    type Output = Sum<Product<T>>;
    fn mul(self, rhs: Self) -> Self::Output {
        let commons: Vec<T> = self
            .iter()
            .filter(|&x| rhs.terms.contains(x))
            .cloned()
            .collect();
        let factors: Sum<Product<T>> = self
            .iter()
            .filter(|&x| !commons.contains(x))
            .flat_map(|x| {
                rhs.iter()
                    .filter(|&y| !commons.contains(y))
                    .map(|y| x.clone() * y.clone())
                    .collect::<Sum<_>>()
            })
            .collect();
        commons
            .into_iter()
            .map(|x| vec![x].into())
            .chain(factors)
            .collect()
    }
}

impl<T> Mul for Sum<Product<T>>
where
    T: std::cmp::PartialEq + std::clone::Clone,
{
    type Output = Sum<Product<T>>;
    fn mul(self, rhs: Self) -> Self::Output {
        let commons: Vec<Product<T>> = self
            .iter()
            .filter(|&x| rhs.terms.contains(x))
            .cloned()
            .collect();
        let factors: Sum<Product<T>> = self
            .iter()
            .filter(|&x| !commons.contains(x))
            .flat_map(|x| {
                rhs.iter()
                    .filter(|&y| !commons.contains(y))
                    .map(|y| x.clone() * y.clone())
                    .collect::<Sum<_>>()
            })
            .collect();
        commons.into_iter().chain(factors).collect()
    }
}

impl<T> Mul for Product<T> {
    type Output = Product<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut factors = self.factors;
        factors.extend(rhs.factors);
        factors.into()
    }
}

impl Mul for Var {
    type Output = Product<Var>;
    fn mul(self, rhs: Self) -> Self::Output {
        vec![self, rhs].into()
    }
}

/// distributes a Vec of Vecs of usize
fn distribute(product: Product<Sum<Var>>) -> Sum<Product<Var>> {
    // (a)(a+b+...) = a
    // (a+b)(a+b+...) = a+b
    // first, remove as much as possible
    let product: Product<Sum<Var>> = product
        .iter()
        .filter(|v| !product.iter().any(|v2| v.contains(v2)))
        .cloned()
        .collect();
    println!("post-reduction: {}", product);
    // progressively distribute, starting with the most similar terms
    let mut expr: Product<Sum<Product<Var>>> = product
        .into_iter()
        .map(|v| v.iter().map(|i| Product::from(vec![*i])).collect())
        .collect();
    println!("expr: {}", expr);
    while expr.len() > 1 {
        // find the most similar element
        let mut min_diff = usize::max_value();
        let mut min_diff_index = (0, 0);
        for (i, elem) in expr.iter().enumerate() {
            for (j, elem2) in expr.iter().enumerate() {
                if i == j {
                    continue;
                }
                let diff = elem.difference(elem2);
                if diff < min_diff {
                    min_diff = diff;
                    min_diff_index = (i, j);
                }
            }
        }
        // distribute the most similar element
        let (i, j) = min_diff_index;
        let mut new_elem = expr[i].clone() * expr[j].clone();
        new_elem.iter_mut().for_each(|v| {
            v.sort_unstable();
            v.dedup();
        });
        new_elem.sort_unstable();
        new_elem.dedup();
        // remove the old elements
        // we need to remove the larger index first to avoid shifting the smaller index
        let (i, j) = if i > j { (i, j) } else { (j, i) };
        expr.remove(i);
        expr.remove(j);
        // and add the new element
        expr.push(new_elem);
    }
    /*
    (a+b)(a+c)(d+e)
    (a+bc)(d+e)
    (ad+ae+bcd+bce)

    */

    let mut sum: Sum<Product<Var>> = expr.pop().unwrap();
    println!("sum: {}", sum);
    sum.sort_unstable();
    sum.dedup();
    // X + XY = X
    sum.iter() // TODO - big slow here very big slow here BAD
        .filter(|v| {
            !sum.iter().any(|v2| {
                v.len() > v2.len()
                    && v2.iter().all(|t2| v.iter().any(|t| t == t2))
                    && v.iter().any(|t| !v2.iter().any(|t2| t == t2))
            })
        })
        .cloned()
        .collect::<Vec<_>>()
        .into()
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
    println!(
        "essential_prime_implicants: {:?}",
        essential_prime_implicants
            .iter()
            .map(|r| r.id.clone())
            .collect::<Vec<_>>()
    );
    // println!("{:?}", petrick);
    println!(
        "uncovered ids: {:?}",
        covered
            .iter()
            .enumerate()
            .filter(|(i, &b)| !b && false_rows.iter().any(|r| r.id[0] == *i))
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
    );
    essential_prime_implicants.append(&mut petricks_method(prime_implicants, covered));
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
        println!(
            "False rows: {:16}{:?}",
            "",
            false_rows.iter().map(|r| &r.id).collect::<Vec<_>>()
        );
        // Step 2: generate prime implicants by combining rows
        let prime_implicants = prime_implicants_from_false_rows(&false_rows);
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
        res.sort_by(|a, b| {
            if a.len() == b.len() {
                a.cmp(b)
            } else {
                a.len().cmp(&b.len())
            }
        });
        let mut res: String = res.concat();
        for _ in 0..essential_prime_implicants.len() - 1 {
            res.push('&');
        }
        res.parse().unwrap() // should never fail
    }
}