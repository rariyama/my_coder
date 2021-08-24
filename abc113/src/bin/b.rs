#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::f64::INFINITY;
use std::iter::FromIterator;
use std::process;
use std::process::exit;
use std::{cmp::min, mem::swap, ops::Mul};

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
        mut n: i64,
        mut t: i64,
        mut a: i64,
        mut h: [i64; n]
    }
    let mut min = INFINITY;
    let mut cnt = 0;
    for (i, &v) in h.iter().enumerate() {
        let x = t as f64 - (v as f64 * 0.006) as f64;
        let diff = (x - a as f64 as f64).abs();

        if diff <= min {
            min = diff;
            cnt = i + 1;
        }
    }

    println!("{}", cnt);
}
