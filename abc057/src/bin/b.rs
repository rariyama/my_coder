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
        n: usize,
        m: usize,
        students: [(i32, i32); n],
        checkpoints: [(i32, i32); m]
    }

    for student in students.iter() {
        let mut max = std::f64::INFINITY;
        let mut cnt = 0;
        for (i, checkpoint) in checkpoints.iter().enumerate() {
            let manhattan: i32 = (student.0 - checkpoint.0).abs() + (student.1 - checkpoint.1).abs();
            if max > manhattan as f64 {
                max = manhattan as f64;
                cnt = i + 1;
            }
        }
        println!("{}", cnt);
    }
}
