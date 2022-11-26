use std::alloc::System;
use std::ffi::c_int;
use std::fs::File;
use std::{fs, io};
use std::io::Read;
use std::os::unix::raw::time_t;
use std::thread::sleep;
use std::time::{Duration, Instant};
use curl::easy::Easy;
use mongodb::{Client, Collection, options::ClientOptions};
use mongodb::{bson::doc, options::FindOptions};
use mongodb::bson::Bson::DateTime;
use mongodb::bson::Document;
use rand::Rng;
// use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use postgres::{Client as psqlClient, NoTls};
use scraper::{Html, Selector};


#[tokio::main]
async fn main() {
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


#[derive(Serialize, Deserialize, Debug)]
struct Page {
    id: i32,
    parent_id: i32,
    title: String,
    body: String,
    comments: i32,
    author_id: i32,
    space_key: String,
}

async fn mongo_get() -> mongodb::error::Result<()> {
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
            id: raw_doc.get_i32("id").unwrap_or(0),
            comments: raw_doc.get_i32("comments").unwrap(),
            author_id: raw_doc.get_i32("week").unwrap_or(0),
            parent_id: raw_doc.get_i32("parent_id").unwrap_or(0),
            title: raw_doc.get_str("title").unwrap_or("").to_string(),
            body: raw_doc.get_str("body").unwrap_or("").to_string(),
            space_key: raw_doc.get_str("space_key").unwrap_or("").to_string(),
        };
        println!("{:?}", page);
    }
    // println!("{}", rand_string(20).to_string());

    Ok(())
}


fn get_pages_psql() {
    //https://docs.rs/postgres/latest/postgres/config/struct.Config.html
    let mut pg_client = psqlClient::connect("host=172.17.0.2 user=dev password=possum dbname=pages", NoTls).unwrap();

    // CREATE
    // create_pages_psql(100, 100, pg_client);

    // GET
    let rows = pg_client.query("select * from pages", &[]).unwrap();
    for row in rows {
        let id: i32 = row.get(0);
        let title: &str = row.get(1);
        let space_key: &str = row.get(2);
        let body: &str = row.get(3);
        //     let data: Option<&[u8]> = row.get(2);

        println!("found page: {:?} {} {} {}", id, title, body, space_key);
    }
}

fn create_pages_psql(from: i32, size: i32, pg_client: &mut postgres::Client) {
    for a in from..size {
        let mut page = Page {
            id: 0,
            title: format!("page {a}").to_string(),
            body: rand_string(20),
            space_key: ["DEV", "TEST", "DEV2"][rand::thread_rng().gen_range(0..2)].to_string(),
            parent_id: rand::thread_rng().gen_range(1..10),
            comments: 2,
            author_id: 1,
        };
        pg_client.execute("insert into pages (title, space_key, body, parent_id, comments, author_id) \
                    values ($1, $2, $3, $4, $5, $6)",
                          &[
                              &page.title.to_owned(),
                              &page.space_key.to_owned(),
                              &page.body.to_owned(),
                              &page.parent_id,
                              &page.comments,
                              &page.author_id,
                          ])
            .unwrap();
    }
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



