use std::{cmp::min, mem::swap, ops::Mul};
use proconio::input; 

fn main() {
    input! {
        mut s: proconio::marker::Chars
    }

    let mut cnt = 0;
    let mut idx = 0;

    let len = s.len();
    for (i, &v) in s.iter().enumerate() {
        if v == 'W' {
            cnt += i - idx;
            idx += 1;
        }

        
    }
    println!("{}", cnt);
}