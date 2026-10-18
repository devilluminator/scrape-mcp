use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScrapeError {
    #[error("`keyword` is required when `search` is true")]
    MissingKeyword,

    #[error("`url` is required when `search` is false")]
    MissingUrl,

    #[error("invalid `url`: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("only http and https URLs are allowed, got `{0}`")]
    UnsupportedScheme(String),

    #[error("host is not allowed (private, loopback or local address): {0}")]
    ForbiddenHost(String),

    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("unsupported content type `{0}`")]
    UnsupportedContentType(String),

    #[error("response body is larger than {0} bytes")]
    BodyTooLarge(usize),
}

pub type ScrapeResult<T> = std::result::Result<T, ScrapeError>;
