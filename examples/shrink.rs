extern crate dotenv;
extern crate pretty_env_logger;
extern crate tinify;

use std::env;
use tinify::Tinify;

fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().unwrap();
    let i = match env::args().nth(1) {
        Some(i) => i,
        None => {
            println!("Usage: shrink <image>");
            return;
        }
    };

    let mut tinify = Tinify::new();
    tinify.shrink(i).unwrap();
}
