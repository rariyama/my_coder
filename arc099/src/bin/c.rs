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
        mut n: i32,
        mut k: i32,
        mut a: [i32; n]
    }

    //(a + (b - 1)) / b
    let l = n - 1 + k - 2;
    println!("{}", l / (k - 1));

}