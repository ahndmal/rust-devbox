use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn matches() {
    let heart_eyed_cat = '😻';
    let mt = 12;
    match mt {
        // Ordering::Less => println!("Too small"),
        Ok(mt) => mt,
        Err(_) => println!("ERROR!"),
        _ => println!("")
    }
}