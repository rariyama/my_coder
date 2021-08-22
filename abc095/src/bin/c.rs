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
        mut a: i64,
        mut b: i64,
        mut c: i64,
        mut x: i64,
        mut y: i64,
    }

    let normal: i64 = a * x + b * y;

    let mut left = (x * 2) * c;
    left += (y - x).max(0) * b;

    let mut right = (y * 2) * c;
    right += (x - y).max(0) * a;

    println!("{}", normal.min(left).min(right));
}
