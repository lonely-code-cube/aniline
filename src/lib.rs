//! Terminal interface and library for d2o5

pub mod client;
pub mod error;
pub mod mal;

pub use client::Client;
pub use error::AnilineError;

#[test]
fn search_test() {
    let client = Client::new();
    println!("{:?}", client.search("Isekai Ojisan"));
}
