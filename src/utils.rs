use crate::AnilineError;
use crate::Client;
use regex::Regex;
use serde::Deserialize;

pub struct AllAnimeQueryResIterator {
    index: usize,
    inner: AllAnimeQueryRes,
}

#[derive(Deserialize, Debug)]
pub struct AllAnimeQueryRes {
    data: A99Data,
}

#[derive(Deserialize, Debug)]
struct A99Data {
    shows: A99Shows,
}

#[derive(Deserialize, Debug)]
struct A99Shows {
    edges: Vec<A99Entity>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct A99Entity {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "availableEpisodes")]
    pub available_episodes: A99Episode,
}

#[derive(Clone, Deserialize, Debug)]
pub struct A99Episode {
    pub sub: usize,
}

impl AllAnimeQueryRes {
    pub fn search(query: &str, client: &Client) -> Result<AllAnimeQueryResIterator, AnilineError> {
        let url = format!(
            "https://allanime.site/allanimeapi?variables={{\"search\":{{\"allowAdult\":true,\"allowUnknown\":true,\"query\":\"{}\"}},\"limit\":40,\"page\":1,\"translationType\":\"{}\",\"countryOrigin\":\"ALL\"}}&extensions={{\"persistedQuery\":{{\"version\":1,\"sha256Hash\":\"9c7a8bc1e095a34f2972699e8105f7aaf9082c6e1ccd56eab99c2f1a971152c6\"}}}}",
            query, "sub"
        );
        let res = client.0.get(&url).send();
        let data = res?.json::<AllAnimeQueryRes>();
        Ok(AllAnimeQueryResIterator {
            index: 0,
            inner: data?,
        })
    }

    pub fn get_all_anime_video_id(
        anime: &A99Entity,
        episode: usize,
        client: &Client,
    ) -> Result<Option<(String, String)>, AnilineError> {
        let url = format!(
            "https://allanime.site/watch/{}/{}/episode-{}-sub",
            anime.id, anime.name, episode
        );
        let res = client.0.get(&url).send();
        let text = res?
            .text()?
            .replace("{", "\n")
            .replace("}", "\n")
            .replace("&referer=", "");
        let re =
            Regex::new(r#".*sourceUrl:\s*".*clock\?id=([^"]*)".*sourceName:\s*"([^"]*)""#).unwrap();
        let cap = re.captures(&text);
        Ok(cap.map(|c| (c[2].to_owned(), c[1].to_owned())))
    }

    pub fn get_video_urls(id: &str, client: &Client) -> Result<Videos, AnilineError> {
        let url = format!("https://blog.allanime.pro/apivtwo/clock.json?id={}", id);
        let res = client.0.get(&url).send();
        let videos = res?.json::<Videos>()?;
        Ok(videos)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Video {
    pub link: String,
    pub hls: Option<bool>,
    pub mp4: Option<bool>,
    #[serde(rename = "resolutionStr")]
    pub res: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Videos {
    pub links: Vec<Video>,
}

impl Iterator for AllAnimeQueryResIterator {
    type Item = A99Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.data.shows.edges.get(self.index);
        self.index += 1;
        item.map(|x| x.clone())
    }
}
