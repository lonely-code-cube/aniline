 use thiserror::Error;

 #[derive(Error, Debug)]
 pub enum AnilineError {
    #[error("Network error")]
    NetworkError(#[from] reqwest::Error),
 }