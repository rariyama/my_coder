use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::f64::INFINITY;
use std::iter::FromIterator;
use std::process;
use std::process::exit;
use std::{cmp::min, mem::swap, ops::Mul};
use itertools::Itertools;


// 最大公約数
fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

// 最小公倍数
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    input! {
        mut s: Chars,
        mut k: i32,
    }
    let ss = s.iter().permutations(s.len());
    let mut v: Vec<Vec<&char>> = Vec::new();
//    println!("{:?}", ss);
    for i in ss {
//        println!("{:?}", i);
        v.push(i);
    }
    v.sort();
    v.dedup();
//    println!("{:?}", v[k as usize - 1]);
    for j in v[k as usize - 1].iter() {
        print!("{}", j);
    }

}
