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
    }
    let mut cnt: i64 = 0;
    for i in 0..n + 1 {
        if 2_i64.pow(i as u32) as i64 <= n as i64 {
            cnt = i;

        } else {
            break
        }
    }
    println!("{}", cnt);

}
