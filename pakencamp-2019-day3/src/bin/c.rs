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
        mut n: i32,
        mut m: i32,
        mut a: [[i32;m];n]
    }

    let mut ans: i64 = 0;

    for i in 0..m {
        for j in i + 1..m {
            let mut sum: i64 = 0;
            for k in &a {
                sum += (k[i as usize].max(k[j as usize])) as i64;
            }
            ans = ans.min(sum);
        }
    }
    println!("{}", ans);
}
