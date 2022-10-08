// mod cor::loops;
// mod loops;
// mod compounds;
// mod reqs;
// mod cor;

use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::Read;
use std::os::unix::raw::time_t;
use std::thread::sleep;
use std::time::Duration;

use curl::easy::Easy;
// use json::JsonValue::String;

fn coll_two() -> usize {
    let mut arr:[i32; 6] = [23,31,12,4234,312,31];
    let mut leng = arr.len();
    arr[0] = 1;
    // arr = [i32; 8];
    let arr2 = &arr;
    let mut arr3 = arr2.clone();
    arr3[0] = 0;
    println!("{:?}", arr);
    println!("{:?}", arr2);
    println!("{:?}", arr3);
    leng = arr.len();
    return leng;
}

fn reading(count: u32) {
    let mut sz: u32 = 0;
    for a in 1..count {
        let mut file = std::fs::File::open(format!("/home/malandr/Documents/lorem{}.txt", a)).unwrap();
        let mut contents = String::new();
        file.read(&mut contents).unwrap();
        print!("{}", contents);
        sz += 1;
    }
    // println!(format!( ">>>> parsed {} files", count));
}

fn main() {
    let now = std::time::SystemTime::now();
    println!("{:?}", now);

    reading(2000);

    let end = std::time::SystemTime::elapsed(&now).unwrap().as_millis().to_string();
    // println!(end);

    // fn concatt(a: &str, b: &str) -> &str {
    //     let c = concat!("{}{}", a, b);
    //         // let c = [a, b].join("");
    //     c
    // }
    // concatt("Hello ", "cat");
    //
    // let mut name: &str = "Murz";
    // name = "Cat";
    // let mut name_st = str::to_string(name);
    // let mut name_2 = name.to_string();
    // name_st.push('A');
    // print!("{} {}", name, name_st);
    //
    // let mut file = std::fs::File::open("src/cor/fs/lorem.txt").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // print!("{}", contents);

    // for a in 1..1000_000 {
    //     println!("num s {}", a);
    // }

    // let n = 6;
    // (0..n).collect::<Vec<u32>>();
    // let nums_arr = (0..n).collect::<Vec<u32>>();
    // for temp in nums_arr {
    //     println!("{}", temp);
    // }


    // positive_sum([1,-4,7,12])

    match std::time::SystemTime::elapsed(&now) {
        Ok(res) => {
            println!("{}", res.as_millis())
        },
        Err(e) => {
            println!("{}", e.to_string())
        }
    }
}

fn closure_refs() {
    print!("{} ", std::mem::size_of::<i32>());
    print!("{} ", std::mem::size_of_val(&12));

    let fl =  File::open("lorem.txt");
    let mut buf = "";
    // match fl {
    //     Ok(data) => {
    //         data.read_to_string(&mut buf);
    //         println!("{:?}", data)},
    //     Err(e) => {println!("ERROR")}
    // }

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


