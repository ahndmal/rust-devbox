
mod http {
    use std::io::Read;
    use std::io::{stdout, Write};
    use curl::easy::Easy;

    // Print a web page onto stdout
    fn main_req() {
        println!("start");

        let mut easy = Easy::new();
        easy.url("https://www.rust-lang.org/").unwrap();
        easy.write_function(|data| {
            stdout().write_all(data).unwrap();
            Ok(data.len())
        }).unwrap();
        easy.perform().unwrap();

        println!("{}", easy.response_code().unwrap());
    }

    fn easy_curl1() {
        let mut easy = Easy::new();
        easy.url("https://www.rust-lang.org/").unwrap();
        easy.write_function(|data| {
            stdout().write_all(data).unwrap();
            Ok(data.len())
        }).unwrap();
        easy.perform().unwrap();

        println!("{}", easy.response_code().unwrap());
    }

    async fn reqw() {
        let body = reqwest::get("https://httpbin.org/get")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("body = {:?}", body);
        println!("END");
    }

// async fn hyper_my() {
//     let client = Client::new();
//     let uri = "http://httpbin.org/ip".parse()?;
//     let resp = client.get(uri).await?;
//     println!("Response: {}", resp.status());
// }

    fn curll() {
        println!("Hello!");
        let mut data = "this is the body".as_bytes();

        let mut easy = Easy::new();
        easy.url("http://www.example.com/upload").unwrap();
        easy.post(true).unwrap();
        easy.post_field_size(data.len() as u64).unwrap();

        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            Ok(data.read(buf).unwrap_or(0))
        }).unwrap();
        transfer.perform().unwrap();
    }

}