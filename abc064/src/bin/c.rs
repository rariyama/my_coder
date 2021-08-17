use std::process;
use std::{cmp::min, mem::swap, ops::Mul};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use proconio::input; 
use proconio::marker::Chars; 
use std::process::exit;


fn main() {
    input! {
        mut n: usize,
        mut a: [i32; n]
    }

    let mut n: HashMap<String, i32> = HashMap::new();
    let mut r: HashMap<String, i32> = HashMap::new();

    r.insert("rainbow".to_string(), 0);
    for i in a.iter() {
        match i {
            1..=399 => {let cnt = n.entry("gray".to_string()).or_insert(0);  *cnt+=1;},
            400..=799 => {let cnt = n.entry("brown".to_string()).or_insert(0); *cnt+=1;},
            800..=1199 => {let cnt = n.entry("green".to_string()).or_insert(0); *cnt+=1;},
            1200..=1599 => {let cnt = n.entry("light_blue".to_string()).or_insert(0); *cnt+=1;},
            1600..=1999 => {let cnt = n.entry("blue".to_string()).or_insert(0); *cnt+=1;},
            2000..=2399 => {let cnt = n.entry("yellow".to_string()).or_insert(0); *cnt+=1;},
            2400..=2799 => {let cnt = n.entry("orange".to_string()).or_insert(0); *cnt+=1;},
            2800..=3199 => {let cnt = n.entry("red".to_string()).or_insert(0); *cnt+=1;},
            3200..=4800 => {let cnt = r.entry("rainbow".to_string()).or_insert(0); *cnt+=1;},
            _ => continue
        };
    }
    let min = match n.iter().len() {
        0 => 1,
        _ => n.iter().len()
    };
    let max = match n.iter().len() {
        0 => *r.get("rainbow").unwrap() as usize,
        _ => min + *r.get("rainbow").unwrap() as usize
    };

    println!("{} {}", min, max);

}
