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
        mut n: i32,
        mut t: [i32; n],
        mut m: i32,
        mut px: [(i32, i32); m]
    }

    for i in px {
        let mut cnt = 0;
        for (idx, j) in t.iter().enumerate() {
            if i.0 == (idx + 1) as i32 {
                cnt += i.1;
            } else {
                cnt += j
            }
        }
        println!("{}", cnt);
    }
}
