use std::io::Read;

pub mod emojis;
pub mod dates_times;
// pub mod coll;
// pub mod compounds;
// pub mod condolee;
// pub mod dbss_ex;
// pub mod gene;
// pub mod funcs;

fn read_lorem() {
    let mut file = std::fs::File::open("src/cor/fs/lorem.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}