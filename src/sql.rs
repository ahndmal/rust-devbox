use chrono::{DateTime, TimeZone, Utc};
use postgres::{Client as psqlClient, NoTls};

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
