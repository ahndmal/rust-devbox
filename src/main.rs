
use std::{fs, io};
use std::alloc::System;
use std::io::Read;
use std::os::unix::raw::time_t;
use std::thread;
use std::time::{Duration, Instant};

use chrono::{DateTime, TimeZone, Utc};
use postgres::{Client as psqlClient, NoTls};
use rand::Rng;
use scraper::{Html, Selector};
// use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};

trait Content {
    fn new(title: &'static str) -> Self;
    fn title(&self) -> &'static str;
}

// #[tokio::main]
fn main()  {
    let now = std::time::SystemTime::now();

    struct Cat {
        id: i32,
        name: String
    };

    pub trait Meow {
        fn meow() {
            println!("meow!");    
        }
        fn who_meows(&self) -> String {
            String::from("Hello")
        }
    }
    
    let murz = Cat {
        id: 123,
        name: "Murz".to_string(),
    };
    
    println!("{:?}", murz.id);

    // end
    println!(">>> action took: {:?} ms", now.elapsed());
}

fn threading() {
    let curr_thread = std::thread::current();
    let thread_id = &curr_thread.id();

    println!("{:?}", curr_thread);
    println!("{:?}", thread_id);

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

async fn parse_html() {
    let mut start = Instant::now();

    let wiki_main_html = reqwest::get("https://en.wikipedia.org/wiki/World_War_I")
        .await.unwrap()
        .text()
        .await.unwrap();

    let html_file = fs::read_to_string("/home/andrii/CLionProjects/rust-features/src/lorem1.html").unwrap();
    let document = Html::parse_document(wiki_main_html.as_str());
    let div_sel = Selector::parse("div").unwrap();
    let li_sel = Selector::parse("li").unwrap();
    let links_sel = Selector::parse("link").unwrap();

    for link in document.select(&links_sel) {
        let links_attrs = &link.value().attrs;
        // for link in links_attrs {
        //     // (QualName { prefix: None, ns: Atom('' type=static), local: Atom('href' type=static) }, Tendril<UTF8>(shared: "//login.wikimedia.org"))
        //     println!("{:?}", link);
        // }
        // println!("{:?}", &link.value().attr("href"));
    }
    let mut li_size = 0;
    for li in document.select(&li_sel) {
        li_size += 1;
        // println!("{:?}", li);
    }
    println!("{:?}", &li_size);

    let end = start.elapsed().as_millis();
    println!(">>> action took {end} millis");
}

fn rand_string(size: i32) -> String {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut \
    labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip \
    ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla \
    pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let words = text.split(" ").collect::<Vec<&str>>();
    let mut lorem = "".to_string();
    for a in 1..size {
        let mut num = rand::thread_rng().gen_range(0..words.len());
        lorem = [lorem, words[num].to_string()].join(" ");
    }
    lorem.to_string()
}
