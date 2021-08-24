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
        mut a: i64,
        mut b: i64,
    }

    let mut arr: Vec<i64> = Vec::new();

    for i in 1..10000 + 1 {
        let s: i64 = (i as f64 * 0.08) as i64;
        let l: i64 = (i as f64 * 0.1) as i64;
        if s == a && l == b {
            arr.push(i);
        }
    }
    //    println!("{:?}", arr);

    if arr.len() == 0 {
        println!("{}", -1);
        exit(0x0100)
    } else {
        println!("{:?}", arr.iter().min().unwrap());
    }
}
