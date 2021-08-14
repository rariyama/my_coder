use std::cmp;
use proconio::input;
 
fn main() {
    input! {
        S: usize,
        T: usize,
    }
    let mut m = cmp::max(S, T);
    let mut mi = cmp::min(S, T);    
    
    if S == 0 || T == 0 {
        mi = m
    }
 
    let mut cnt = 0;
    for i in 0..mi + 1{
        for j in 0..mi + 1 {
            for k in 0..mi + 1 {
                if i + j + k <= S && i * j * k <= T {
                    cnt += 1;
                }
                if i + j + k > S || i * j * k > T {
                    break;
                }
            }
        }
    }
println!("{:?}", cnt);
 
//    println!("{}", v.iter().min());
}

