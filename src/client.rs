use crate::error::AnilineError;
use crate::mal::{anime::Anime, AnimeSearchResult};
use crate::utils::{A99Entity, AllAnimeQueryRes, AllAnimeQueryResIterator, Videos};

pub struct Client(pub reqwest::blocking::Client);

impl Client {
    pub fn new() -> Self {
        Self(
            reqwest::blocking::Client::builder()
                .user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 10_3 like Mac OS X)")
                .build()
                .unwrap(),
        )
    }

    pub fn search(&self, query: &str) -> Result<AnimeSearchResult, AnilineError> {
        Anime::search(query, self)
    }

    pub fn get_all_anime_query_res(
        &self,
        query: &str,
    ) -> Result<AllAnimeQueryResIterator, AnilineError> {
        AllAnimeQueryRes::search(query, self)
    }

    pub fn get_all_anime_video_id(
        &self,
        anime: &A99Entity,
        episode: usize,
    ) -> Result<Option<(String, String)>, AnilineError> {
        AllAnimeQueryRes::get_all_anime_video_id(anime, episode, self)
    }

    pub fn get_video_urls(&self, id: &str) -> Result<Videos, AnilineError> {
        AllAnimeQueryRes::get_video_urls(id, self)
    }
}
