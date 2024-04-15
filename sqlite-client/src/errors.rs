#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ConnectionError")]
    ConnectionError(#[source] rusqlite::Error),
    #[error("{0}")]
    Other(&'static str),
}
