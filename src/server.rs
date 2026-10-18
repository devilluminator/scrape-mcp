use std::sync::Arc;

use rmcp::{
    ServerHandler,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

use crate::config::Config;
use crate::error::ScrapeResult;
use crate::extract::extract_page;
use crate::fetch::fetch_html;
use crate::page::Page;
use crate::params::ExtractParams;
use crate::target;

#[derive(Clone)]
pub struct ScraperServer {
    config: Arc<Config>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl ScraperServer {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Fetch a web page and return its structured content: the title, the top \
            heading, the paragraph texts, and the links with absolute hrefs. \
            Set `search` to true to run a Brave web search for `keyword`; the result \
            is a `results` list of {title, url, snippet}. Leave `search` false to \
            scrape `url` directly."
    )]
    async fn web_extract(&self, Parameters(params): Parameters<ExtractParams>) -> String {
        match self.run(params).await {
            Ok(page) => {
                serde_json::to_string_pretty(&page).unwrap_or_else(|e| error_json(&e.to_string()))
            }
            Err(e) => error_json(&e.to_string()),
        }
    }
}

#[tool_handler]
impl ServerHandler for ScraperServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Web scraping tool: `web_extract` returns the title, heading, paragraphs \
                and links of a URL, or of a Brave search results page."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

impl ScraperServer {
    async fn run(&self, params: ExtractParams) -> ScrapeResult<Page> {
        let url = target::resolve(&params, self.config.allow_private_hosts)?;
        let html = fetch_html(&url).await?;
        Ok(extract_page(&html, &url, &self.config.limits))
    }
}

fn error_json(msg: &str) -> String {
    serde_json::json!({ "error": msg }).to_string()
}
