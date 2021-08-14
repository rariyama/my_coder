use std::cmp;
use proconio::input;
 
fn main() {
    input! {
        N: usize
    }

    if 1 <= N && N <= 125 {
        println!("{}", 4);
    } else if 126 <= N && N <= 211 {
        println!("{}", 6);
    } else if 212 <= N {
        println!("{}", 8);
    }
}