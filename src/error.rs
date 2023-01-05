use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnilineError {
    #[error("Network error")]
    NetworkError(#[from] reqwest::Error),
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("Subprocess Error")]
    SubprocessError(#[from] subprocess::PopenError),
}
