extern crate hyper;
extern crate pretty_env_logger;
extern crate rand;

use std::env;

use hyper::Client;
// use rand::Rng;

fn main() {
    pretty_env_logger::init().unwrap();

    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return;
        }
    };

    infinite_request(url)
}

fn infinite_request(url: String) {
    let client = Client::new();
    let mut count = 0;

    loop {
        let info_url = url.parse::<hyper::Url>().unwrap();
        let res = client.post(info_url)
            .body("pay amount=420")
            .send();

        match res {
            Ok(res) => {
                println!("Response: {}", res.status)
            },
            Err(e) => println!("Err: {:?}", e)
        }

        count = count + 1;
        println!("Count: {}", count);
    }
}
