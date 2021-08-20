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
        mut d: i64,
    }

    let len = b - a + 1;

    let cdiv = b / c - (a - 1) / c;
    let ddiv = b / d - (a - 1) / d;
    let g = lcm(c as usize, d as usize) as i64;
    let common = b / g - (a - 1) / g;

    println!("{}", len - cdiv - ddiv + common);
}
