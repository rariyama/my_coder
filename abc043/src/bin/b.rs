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
        mut s: Chars
    }
    let mut ans: Vec<char> = Vec::new();

    for i in s {
        match i {
            'B' => {
                if ans.iter().len() == 0 {
                    continue;
                } else {
                    ans.pop();
                }
            }
            _ => ans.push(i),
        }
    }
    for j in ans.iter() {
        print!("{}", j);
    }
}
