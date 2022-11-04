use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn matches() {
    //https://apps.timwhitlock.info/emoji/tables/unicode
    let heart_eyed_cat = '😻';
    let smile1 = '😊';
    let smiles: String = "😼 😺 ✂ ✏ ❗ ❌  ✔   ";
    let mt = 12;
    match mt {
        // Ordering::Less => println!("Too small"),
        Ok(mt) => mt,
        Err(_) => println!("ERROR!"),
        _ => println!("")
    }
}