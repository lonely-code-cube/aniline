//! Terminal interface and library for d2o5

pub mod client;
pub mod error;
pub mod mal;
mod utils;

pub use client::Client;
pub use error::AnilineError;
pub use utils::{Videos, Video};

#[test]
fn search_test() {
    let client = Client::new();
    println!("{:?}", client.search("Isekai Ojisan"));
}
