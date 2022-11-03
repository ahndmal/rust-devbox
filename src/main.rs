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
use mongodb::{Client, Collection, options::ClientOptions};
use mongodb::{bson::doc, options::FindOptions};
use mongodb::bson::Document;
use rand::Rng;
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
    struct Page {
        id: i64,
        parent_id: i64,
        title: String,
        body: String,
        comments: i32,
        author_id: i64,
    }

    let db_url = format!("mongodb+srv://{user}:{pass}@cluster0.t1yi6.mongodb.net/?retryWrites=true&w=majority",
                         user = std::env::var("USER").unwrap(),
                         pass = std::env::var("PASS").unwrap());

    // let mut client_options = ClientOptions::parse(db_url).await?;
    // client_options.app_name = Some("My App".to_string());
    // let client = Client::with_options(client_options)?;
    let client = Client::with_uri_str(db_url).await?;
    let db_name = "wiki";
    let db = client.database(db_name);

    let pages_coll = client
        .database(db_name)
        .collection::<Document>("pages"); // collection::<Page>

    let filter = doc! { "parent_id": 0 };
    // let find_options = FindOptions::builder().sort(doc! {"record": -1}).build();

    // let mut cursor =  workouts_coll.find(None, find_options).await?;
    let mut cursor = pages_coll.find(None, None).await?;

    // CREATE
    // create_pages(pages_coll, 201, 1000).await;

    // DELETE!
    // pages_coll.delete_many(doc! {}, None).await?;

    while cursor.advance().await? {
        let raw_doc = cursor.current();
        // let mut comment = match raw_doc.get_str("comments") {
        //     Ok => raw_doc.get_str("commets").unwrap(),
        //     Err => "",
        // };
        let mut page = Page {
            id: raw_doc.get_i64("id").unwrap_or(0),
            comments: raw_doc.get_i32("comments").unwrap(),
            author_id: raw_doc.get_i64("week").unwrap_or(0),
            parent_id: raw_doc.get_i64("parent_id").unwrap_or(0),
            title: raw_doc.get_str("title").unwrap_or("").to_string(),
            body: raw_doc.get_str("body").unwrap_or("").to_string(),
        };
        println!("{:?}", page);
    }


    // println!("{}", rand_string(20).to_string());

    Ok(())
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

async fn create_pages(pages_coll: Collection<Document>, from: i32, size: i32) {
    for a in from..=size {
        let mut new_page = doc! {
            "id": i64::from(a),
            "parent_id": i64::from(1),
            "title": format!("page {a}").to_string(),
            "body":  rand_string(30).to_string(),
            "comments": i32::from(1),
            "author_id": i64::from(1)
        };
        pages_coll.insert_many(vec![new_page], None).await.unwrap();
    }
}



