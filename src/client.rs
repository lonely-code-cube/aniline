use crate::mal::{anime::Anime, AnimeSearchResult};
use crate::error::AnilineError;

pub struct Client(pub reqwest::blocking::Client);

impl Client {
    pub fn new() -> Self {
        Self(reqwest::blocking::Client::new())
    }

    pub fn search(&self, query: &str) -> Result<AnimeSearchResult, AnilineError> {
        Anime::search(query, self)
    }
}
