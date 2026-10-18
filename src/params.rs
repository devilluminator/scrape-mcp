use rmcp::schemars;
use serde::Deserialize;

/// Arguments of the `web_extract` tool (the JSON Schema is generated from this).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExtractParams {
    /// If true, run a Brave web search for `keyword` instead of using `url`.
    #[serde(default)]
    pub search: bool,

    /// Search query. Required when `search` is true.
    pub keyword: Option<String>,

    /// Absolute http(s) URL to scrape. Required when `search` is false.
    pub url: Option<String>,
}
