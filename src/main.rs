// mod cor::loops;
// mod loops;
// mod compounds;
// mod reqs;

use std::fs::File;
use std::io;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

use curl::easy::Easy;
use json::JsonValue::String;

fn main() {
    // let n = 6;
    // (0..n).collect::<Vec<u32>>();
    // let nums_arr = (0..n).collect::<Vec<u32>>();
    // for temp in nums_arr {
    //     println!("{}", temp);
    // }

    println!("{}", str::to_uppercase("aaa"));

    // let a = 1;
    // let b = 4;
    // let mut anums: Vec<u32> = vec![];
    // for i in a..b {
    //     nums.push(i);
    // }

    // positive_sum([1,-4,7,12])

    print!("{} ", std::mem::size_of::<i32>());
    print!("{} ", std::mem::size_of_val(&12));

    let fl =  File::open("lorem.txt");
    let mut buf = "";
    match fl {
        Ok(data) => {
            data.read_to_string(&mut buf);
            println!("{}", )},
        Err(e) => {println!("ERROR")}
    }



}

fn positive_sum(slice: &[i32]) -> i32 {
    // let mut nums: Vec<i32> = vec![];
    let mut summ = 0;
    for a in slice {
        if a > &0 {
            summ += a;
        }
    }
    return summ;

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

// fn string_to_array(s: &str) -> Vec<String> {
//     let words: Vec<String> = s.split(" ").collect();
//     return words;
// }

fn arr(n: usize) -> Vec<u32> {
    // let mut nums: Vec<u32> = vec![];
    // for a in 0..n {
    //     nums.push(a);
    // }
    // return nums;
    //let st = "I love Rust";
    // println!("{:?}", st.split(" ").collect::<Vec<&str>>());
    return vec![];
}


