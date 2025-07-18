use mongodb::{Client, Collection, options::ClientOptions};
use mongodb::{bson::doc, options::FindOptions};
use mongodb::bson::Bson::DateTime as MongoDateTime;
use mongodb::bson::Document;


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
            author_id: raw_doc.get_i32("week").unwrap_or(0),
            parent_id: raw_doc.get_i32("parent_id").unwrap_or(0),
            title: raw_doc.get_str("title").unwrap_or("").to_string(),
            body: raw_doc.get_str("body").unwrap_or("").to_string(),
            space_key: raw_doc.get_str("space_key").unwrap_or("").to_string(),
            created_at: None, //raw_doc.get_datetime("created_at"),
            last_updated: None, //raw_doc.get_datetime("last_updated"),

        };
        println!("{:?}", page);
    }
    // println!("{}", rand_string(20).to_string());

    Ok(())
}
