mod matches;
mod web;
mod loops;
mod compounds;

use std::io;


fn main() {
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

