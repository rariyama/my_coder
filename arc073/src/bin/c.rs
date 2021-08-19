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
        mut t: i32,
        mut arr: [i32; n],
    }

// println!("{} {} {:?}", n, t, arr);

    let mut cnt = 0;
    for i in 0..arr.iter().len() - 1 {
        let x = arr[i + 1] - arr[i];
        cnt += x.min(t);
    }
    println!("{}", cnt + t);
}
