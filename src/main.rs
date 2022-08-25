mod matches;
mod web;
mod loops;
mod compounds;
mod reqs;
// mod win11;

use std::io;
use std::io::Read;
use curl::easy::Easy;
// use hyper::Client;

fn main(){
    reqw();
}

async fn reqw() {
    println!("Started!");
    let body = reqwest::get("https://httpbin.org/get")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("body = {:?}", body);
    println!("END");
}

// async fn hyper_my() {
//     let client = Client::new();
//     let uri = "http://httpbin.org/ip".parse()?;
//     let resp = client.get(uri).await?;
//     println!("Response: {}", resp.status());
// }

fn curll() {
    println!("Hello!");
    let mut data = "this is the body".as_bytes();

    let mut easy = Easy::new();
    easy.url("http://www.example.com/upload").unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap();
}


// fn main() {

    // let name = "Vasyl";
    //
    // println!("Hello {}!", name);
    // println!("Enter your guess:");
    //
    // let mut guess = String::new();
    //
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line!");
    //
    // let length = String::len(&guess);
    // println!("length is {}", length);
    //
    // let guess: u32 = guess.trim().parse().expect("please type a number");
    //
    //
    // println!("You guessed: {}", guess);

// }

