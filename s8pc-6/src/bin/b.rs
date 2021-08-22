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
        mut ab: [[i64; 2]; n],
    }

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for i in &ab {
        left.push(i[0]);
        right.push(i[1]);
    }

    println!("{:?} {:?}", left, right);

    let mut ans = INFINITY;
    for i in left {
        for j in &right {
            let mut cnt = 0;
            for k in &ab {
                let leftC: i64 = (i - k[0]).abs() as i64;
                let middleC: i64 = (k[1] - k[0]).abs() as i64;
                let rightC: i64 = (j - k[1]).abs() as i64;
                cnt += leftC + middleC + rightC;
            }
            ans = ans.min(cnt as f64);
        }
    }
    println!("{}", ans);
}
