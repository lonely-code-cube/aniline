//! This module contains functions and structures to interact with [myanimelist.net](https://myanimelist.net)/Jikan API.
//!
//! These fucntions are not intended to be called independently.
//! The [crate::client::Client] is the intended way to access these functions.
static BASE_URL: &str = "https://api.jikan.moe/v4";

use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
pub struct AnimeSearchResult {
    pub data: Vec<anime::Anime>,
    pub pagination: Pagination,
}

pub mod anime {
    use crate::{error::AnilineError, mal::BASE_URL, Client};
    use serde::Deserialize;

    #[derive(Clone, Debug, Deserialize)]
    pub struct Anime {
        pub mal_id: usize,
        pub url: String,
        pub images: Option<Images>,
        pub trailer: Option<Trailer>,
        pub approved: bool,
        pub titles: Option<Vec<Title>>,
        pub title: String,
        pub title_english: Option<String>,
        pub title_japanese: Option<String>,
        pub title_synonyms: Option<Vec<String>>,
        pub r#type: Option<String>,
        pub source: Option<String>,
        pub episodes: Option<usize>,
        pub status: Option<String>,
        pub airing: Option<bool>,
        pub aired: Option<Aired>,
        pub duration: Option<String>,
        pub rating: Option<String>,
        pub score: Option<f64>,
        pub scored_by: Option<usize>,
        pub rank: Option<usize>,
        pub popularity: Option<usize>,
        pub members: Option<usize>,
        pub favorites: Option<usize>,
        pub synopsis: Option<String>,
        pub background: Option<String>,
        pub season: Option<String>,
        pub year: Option<usize>,
        pub broadcast: Option<Broadcast>,
        pub producers: Option<Vec<Producer>>,
        pub lisensors: Option<Vec<Licensor>>,
        pub studios: Option<Vec<Studio>>,
        pub genres: Option<Vec<Genre>>,
        pub explicit_genres: Option<Vec<ExplicitGenre>>,
        pub themes: Option<Vec<Theme>>,
        pub demographics: Option<Vec<Demographic>>,
    }

    impl Anime {
        pub fn search(
            query: &str,
            client: &Client,
        ) -> Result<super::AnimeSearchResult, AnilineError> {
            let url = format!("{}/anime?q={}", BASE_URL, query);
            let res = client.0.get(&url).send();
            let data = res?.json::<super::AnimeSearchResult>();
            Ok(data?)
        }
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Images {
        pub jpg: Option<Image>,
        pub webp: Option<Image>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Image {
        pub image_url: Option<String>,
        pub small_image_url: Option<String>,
        pub large_image_url: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Trailer {
        pub youtube_id: Option<String>,
        pub url: Option<String>,
        pub embed_url: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Title {
        pub r#type: Option<String>,
        pub title: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct AiredPropTime {
        pub day: Option<usize>,
        pub month: Option<usize>,
        pub year: Option<usize>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct AiredProp {
        pub from: Option<AiredPropTime>,
        pub to: Option<AiredPropTime>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Aired {
        pub from: Option<String>,
        pub to: Option<String>,
        pub prop: AiredProp,
        pub string: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Broadcast {
        pub date: Option<String>,
        pub time: Option<String>,
        pub timezone: Option<String>,
        pub string: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Producer {
        pub mal_id: usize,
        pub r#type: Option<String>,
        pub name: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Licensor {
        pub mal_id: usize,
        pub r#type: Option<String>,
        pub name: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Studio {
        pub mal_id: usize,
        pub r#type: Option<String>,
        pub name: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Genre {
        pub mal_id: usize,
        pub r#type: Option<String>,
        pub name: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct ExplicitGenre {
        pub mal_id: usize,
        pub r#type: Option<String>,
        pub name: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Theme {
        pub mal_id: usize,
        pub r#type: Option<String>,
        pub name: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Demographic {
        pub mal_id: usize,
        pub r#type: Option<String>,
        pub name: Option<String>,
        pub url: Option<String>,
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct PaginationItems {
    pub count: usize,
    pub total: usize,
    pub per_page: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pagination {
    pub last_visible_page: usize,
    pub has_next_page: bool,
    pub items: PaginationItems,
}
