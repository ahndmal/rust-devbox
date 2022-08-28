// mod cor::loops;
// mod loops;
// mod compounds;
// mod reqs;

use std::io;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use std::io::{stdout, Write};

use curl::easy::Easy;

fn main() {

    let nums = count_by(2,5);
    for n in nums {
        println!("{}", n);
    }


}

fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut nums = vec![];
    for a in 1..n + 1 {
        let mut b = a * x;
        nums.push(b);
    }
    return  nums;
}

fn count_by2(x: u32, n: u32) -> Vec<u32>  {
    (1..=n).map(|e| x*e).collect()
    //(x..x*n+1).filter(|y| *y%x==0).collect::<Vec<u32>>()
}


