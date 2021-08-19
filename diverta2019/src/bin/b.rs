use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::f64::INFINITY;
use std::iter::FromIterator;
use std::process;
use std::process::exit;
use std::{cmp::min, mem::swap, ops::Mul};

fn main() {
    input! {
        mut r: i32,
        mut g: i32,
        mut b: i32,
        mut n: i32,
    }
    let mut cnt = 0;

    for i in 0..3001 {
        for j in 0..3001 {
            let x = i * r + j * g;
            if n >= x && (n - x) % b == 0 {
                cnt += 1
            }
        }
    }
    println!("{}", cnt);
}

