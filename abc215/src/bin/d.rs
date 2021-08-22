use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::f64::INFINITY;
use std::iter::FromIterator;
use std::process;
use std::process::exit;
use std::{cmp::min, mem::swap, ops::Mul};
use itertools::Itertools;



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


fn is_prime(n: u32) -> bool {
    if n < 2 {
        false
    } else if n == 2 {
        true
    } else if n % 2 == 0 {
        false
    } else {
        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}

fn main() {
    input! {
        mut n: i32,
        mut m: i32,
        mut a: [i32; n]
    }
    let mut cnt = 1;
    let mut arr:Vec<i32> = Vec::new();
    arr.push(1);
//    let x = primal::estimate_nth_prime( as u64);
    for i in (2..m).filter(|&x| is_prime(x as u32) == true) {
        for (idx, &j) in a.iter().enumerate() {
            let gc = gcd(i as usize, j as usize);
            if gc != 1 {
                break
            }
            if idx == a.len() -1 {
                cnt += 1;
                arr.push(i);
            }
        }
    }
    println!("{}", cnt);
    for k in arr.iter() {
        println!("{}", k);
    }
}
