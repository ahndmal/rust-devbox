use std::{fs, io};
use std::alloc::System;
use std::ffi::c_int;
use std::fs::File;
use std::io::Read;
use std::os::unix::raw::time_t;
use std::thread;
use std::time::{Duration, Instant};

use chrono::{DateTime, TimeZone, Utc};
use curl::easy::Easy;
use mongodb::{Client, Collection, options::ClientOptions};
use mongodb::{bson::doc, options::FindOptions};
use mongodb::bson::Bson::DateTime as MongoDateTime;
use mongodb::bson::Document;
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
        fn who_meows(&self) {
            
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


fn run_get_pages_psql() {
    let db_host = std::env::var("DB_HOST").unwrap();
    let db_pass = std::env::var("DB_PASS").unwrap();
    //https://docs.rs/postgres/latest/postgres/config/struct.Config.html
    let mut pg_client =
        psqlClient::connect(format!("host={db_host} user=dev password={db_pass} dbname=wiki1").as_str(),
                            NoTls).unwrap();

    // CREATE
    // create_pages_psql(1000, 100, &mut pg_client);

    // get pages
    get_pages_psql(&mut pg_client);
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


#[derive(Serialize, Deserialize, Debug)]
struct Page {
    id: i32,
    title: String,
    body: String,
    space_key: String,
    author_id: i32,
    parent_id: i32,
    created_at: String, //chrono::DateTime<Utc>, //postgres::types::Timestamp<>
    last_updated: String // chrono::DateTime<Utc>, //postgres::types::Timestamp<>
}

// async fn mongo_get() -> mongodb::error::Result<()> {
//     let db_url = format!("mongodb+srv://{user}:{pass}@cluster0.t1yi6.mongodb.net/?retryWrites=true&w=majority",
//                          user = std::env::var("USER").unwrap(),
//                          pass = std::env::var("PASS").unwrap());
//
//     // let mut client_options = ClientOptions::parse(db_url).await?;
//     // client_options.app_name = Some("My App".to_string());
//     // let client = Client::with_options(client_options)?;
//     let client = Client::with_uri_str(db_url).await?;
//     let db_name = "wiki";
//     let db = client.database(db_name);
//
//     let pages_coll = client
//         .database(db_name)
//         .collection::<Document>("pages"); // collection::<Page>
//
//     let filter = doc! { "parent_id": 0 };
//     // let find_options = FindOptions::builder().sort(doc! {"record": -1}).build();
//
//     // let mut cursor =  workouts_coll.find(None, find_options).await?;
//     let mut cursor = pages_coll.find(None, None).await?;
//
//     // CREATE
//     // create_pages(pages_coll, 201, 1000).await;
//
//     // DELETE!
//     // pages_coll.delete_many(doc! {}, None).await?;
//
//     while cursor.advance().await? {
//         let raw_doc = cursor.current();
//         // let mut comment = match raw_doc.get_str("comments") {
//         //     Ok => raw_doc.get_str("commets").unwrap(),
//         //     Err => "",
//         // };
//         let mut page = Page {
//             id: raw_doc.get_i32("id").unwrap_or(0),
//             author_id: raw_doc.get_i32("week").unwrap_or(0),
//             parent_id: raw_doc.get_i32("parent_id").unwrap_or(0),
//             title: raw_doc.get_str("title").unwrap_or("").to_string(),
//             body: raw_doc.get_str("body").unwrap_or("").to_string(),
//             space_key: raw_doc.get_str("space_key").unwrap_or("").to_string(),
//             created_at: None, //raw_doc.get_datetime("created_at"),
//             last_updated: None, //raw_doc.get_datetime("last_updated"),
//
//         };
//         println!("{:?}", page);
//     }
//     // println!("{}", rand_string(20).to_string());
//
//     Ok(())
// }

fn get_pages_psql(pg_client: &mut postgres::Client) {

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
            author_id: 1,
            created_at: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap().to_string(),
            last_updated: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap().to_string(),
        };
        pg_client.execute("insert into pages (id, title, body, space_key, parent_id, author_id, created_at, last_updated) \
                    values ($1, $2, $3, $4, $5, $6, $7, $8)",
                          &[
                              &page.id,
                              &page.title.to_owned(),
                              &page.body.to_owned(),
                              &page.space_key.to_owned(),
                              &page.parent_id,
                              &page.author_id,
                              &page.created_at,
                              &page.last_updated,
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



