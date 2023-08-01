mod my_db {

    use postgres::{Client as psqlClient, NoTls};

    fn get_conn() {
        let db_host = std::env::var("DB_HOST").unwrap();
        let db_pass = std::env::var("DB_PASS").unwrap();
        //https://docs.rs/postgres/latest/postgres/config/struct.Config.html
        let mut pg_client =
            psqlClient::connect(format!("host={db_host} user=dev password={db_pass} dbname=wiki1").as_str(),
                                NoTls).unwrap();
    }
}
