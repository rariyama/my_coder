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
        mut s: String
    }

    for i in 0..s.len() {
        for j in i + 1..s.len() {
            let start = &s[..i + 1];
            let end = &s[j..s.len()];
//            let mut x = String::from(start) + &String::from(end);
            let x = format!("{}{}", start, end);
            if x == "keyence" {
                println!("YES");
                exit(0x0100);
            }
        }
    }

    println!{"NO"};
}
