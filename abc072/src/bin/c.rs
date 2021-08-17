use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::iter::FromIterator;
use std::process;
use std::process::exit;
use std::{cmp::min, mem::swap, ops::Mul};

fn main() {
    input! {
        mut n: usize,
        mut a: [i32; n]
    }

    let mut ttl: HashMap<String, i32> = HashMap::new();
    for i in a.iter() {
        let cnt = ttl.entry(i.to_string()).or_insert(0);
        *cnt += 1;
        let cnt = ttl.entry((i + 1).to_string()).or_insert(0);
        *cnt += 1;
        let cnt = ttl.entry((i - 1).to_string()).or_insert(0);
        *cnt += 1;
    }

    let max = ttl.iter().max_by(|k, v| k.1.cmp(&v.1)).unwrap();
    println!("{}", max.1);
}
