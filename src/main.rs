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
    let _a = [""; 0];
    let _aa = vec![true; 0];
    let _b = vec![false; 0];

    let mut ages = vec![1,2,3; 1000];
    for a in ages {
        println!("a is {}", a)
    }
}

fn cats_loop() {
    let mut x = [4; 5000];
    x[2000] = 14;
    print!("{}, {} \n", x[1000], x[2000]);

    let mut cats = vec!["Murz", "Lyvko", "Sapko"];
    cats.push("Pukh");
    for cat in cats {
        println!("{}", cat);
    }
}

fn guess() {
    let name = "Vasyl";
    println!("Hello {}!", name);
    println!("Enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let length = String::len(&guess);
    println!("length is {}", length);

    let guess: u32 = guess.trim().parse().expect("please type a number");

    println!("You guessed: {}", guess);
}

