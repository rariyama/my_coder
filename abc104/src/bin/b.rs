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
        mut s: Chars,
    }

    let mut cnt = 0;
    let mut ucnt = 0;
    for (i, &c) in s.iter().enumerate() {
        // condition 1
        if i == 0 && c != 'A' {
            println!("WA");
            exit(0x0100);
        }
        // condition 2
        if 2 <= i && i <= s.iter().len() - 2 && s[i] == 'C' {
            cnt += 1;
            if 1 < cnt {
                println!("WA");
                exit(0x0100);
            } else {
                continue;
            }
        }
        if cnt == 0 && i == s.len() - 2 {
            println!("WA");
            exit(0x0100);
        }

        // condition 3
        if (1 == i && s[i].is_uppercase() == true)
            || (2 <= i && i < s.len() - 1 && s[i] != 'C' && s[i].is_uppercase() == true)
            || (i == s.len() - 1 && s[i].is_uppercase() == true)
        {
            println!("WA");
            exit(0x0100);
        }
    }
    println!("AC");
}
