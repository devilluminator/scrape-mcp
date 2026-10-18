use reqwest::Client;
use url::Url;

use crate::error::ScrapeResult;

pub async fn fetch_html(url: &Url) -> ScrapeResult<String> {
    let client = Client::new();

    let html = client
        .get(url.as_str())
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
            (KHTML, like Gecko) Chrome/120.0 Safari/537.36",
        )
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(html)
}
