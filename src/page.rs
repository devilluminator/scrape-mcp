use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Link {
    pub text: String,
    pub href: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

#[derive(Debug, Serialize)]
pub struct Page {
    pub source_url: String,
    pub title: Option<String>,
    pub header: Option<String>,
    /// Filled for search pages; replaces `paragraphs` and `links` there.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<SearchResult>,
    pub paragraphs: Vec<String>,
    pub links: Vec<Link>,
    pub truncated: bool,
}
