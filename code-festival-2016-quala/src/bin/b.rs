
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
        mut a: [i64; n],
    }

    let mut cnt = 0;
    for i in 0..n {
        if i + 1 == a[(a[i as usize] - 1) as usize] {
            cnt += 1;
        }
    }
    println!("{}", cnt / 2);

    // 全探索
    //    let mut h: Vec<(usize, i64)> = Vec::new();
    //    for (i, &v) in a.iter().enumerate() {
    //        h.push((i + 1 , v));
    //    }
    //
    //    let mut cnt = 0;
    //    for i in 0..h.len() {
    //        for j in i + 1..h[i].1 as usize {
    //            if h[i].0 as i64 == h[j].1 && h[i].1 == h[j].0 as i64 {
    //                cnt += 1;
    //            }
    //        }
    //    }
    //    println!("{}", cnt);
}
