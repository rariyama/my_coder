use std::process;
use std::{cmp::min, mem::swap, ops::Mul};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use proconio::input; 

fn main() {
    input! {
        mut n: usize,
        mut s: proconio::marker::Chars
    }

    let mut ans = 0;
    for i in 1..s.iter().len() {
        let left: HashSet<&char> = s[0..i].into_iter().collect();
        let right: HashSet<&char> = s[i..].into_iter().collect();
        let mut cnt = 0;
        for &i in left.iter() {
            match  right.iter().find(|&&x| x == i) {
                Some(_) => cnt += 1,
                _ => continue
            };
        }
        ans = ans.max(cnt);
    }
    println!("{}", ans);
}
