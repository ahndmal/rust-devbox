// mod cor::loops;
// mod loops;
// mod compounds;
// mod reqs;

use std::alloc::System;
use std::fs::File;
use std::io;
use std::io::Read;
use std::os::unix::raw::time_t;
use std::thread::sleep;
use std::time::Duration;

use curl::easy::Easy;
use mongodb::{Client, options::ClientOptions};
use mongodb::{bson::doc, options::FindOptions};
// use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};

fn read_lorem() {
    let mut file = std::fs::File::open("src/cor/fs/lorem.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}


#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    #[derive(Serialize, Deserialize, Debug)]
    struct Workout {
        record: i64,
        sets: i32,
        comments: String,
        day: String,
    }

    let db_url = format!("mongodb+srv://{user}:{pass}@cluster0.t1yi6.mongodb.net/?retryWrites=true&w=majority",
                         user = std::env::var("USER").unwrap(),
                         pass = std::env::var("PASS").unwrap());

    // let mut client_options = ClientOptions::parse(db_url).await?;
    // client_options.app_name = Some("My App".to_string());
    // let client = Client::with_options(client_options)?;
    let client = Client::with_uri_str(db_url).await?;
    let db = client.database("workouts");

    let workouts_coll = client
        .database("workouts")
        .collection::<Workout>("workouts");

    let filter = doc! { "day": "Sunday" };
    let find_options = FindOptions::builder().sort(doc! {"record": -1}).build();

    // let mut cursor =  workouts_coll.find(None, find_options).await?;
    let mut cursor = workouts_coll.find(None, None).await?;

    while cursor.advance().await? {
        let raw_doc = cursor.current();
        // let mut comment = match raw_doc.get_str("comments") {
        //     Ok => raw_doc.get_str("commets").unwrap(),
        //     Err => "",
        // };
        let mut work = Workout{
            record: raw_doc.get_i64("record").unwrap_or(0),
            sets: raw_doc.get_i32("sets").unwrap(),
            comments: raw_doc.get_str("comments").unwrap_or("").to_string(),
            day: raw_doc.get_str("day").unwrap().to_string()
        };
        println!("{:?}", work);
    }

    Ok(())
}

fn closure_refs() {
    // print!("{} ", std::mem::size_of::<i32>());
    // print!("{} ", std::mem::size_of_val(&12));
    //
    // let fl =  File::open("lorem.txt");
    // let mut buf = "";
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
    return nums;
}

fn count_by2(x: u32, n: u32) -> Vec<u32> {
    (1..=n).map(|e| x * e).collect()
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


