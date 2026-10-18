use std::collections::HashSet;

use scraper::Html;
use url::Url;

use super::links::resolve;
use super::selectors;
use super::text::text_of;
use crate::config::Limits;
use crate::page::SearchResult;

/// Pull organic results out of a Brave results page. The bool is `true` when
/// the cap cut the list short.
pub(super) fn collect(doc: &Html, base: &Url, limits: &Limits) -> (Vec<SearchResult>, bool) {
    let mut seen = HashSet::new();
    let mut results = Vec::new();

    for block in doc.select(&selectors::RESULT) {
        // The first usable link in the block is the result link.
        let Some((link_el, url)) = block.select(&selectors::A).find_map(|a| {
            let url = a.value().attr("href").and_then(|h| resolve(base, h))?;
            Some((a, url))
        }) else {
            continue;
        };

        if !seen.insert(url.clone()) {
            continue;
        }
        if results.len() >= limits.max_links {
            return (results, true);
        }

        let title = block
            .select(&selectors::RESULT_TITLE)
            .next()
            .map(|el| text_of(el, limits.max_chars))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| text_of(link_el, limits.max_chars));

        let snippet = block
            .select(&selectors::RESULT_SNIPPET)
            .next()
            .map(|el| text_of(el, limits.max_chars))
            .unwrap_or_default();

        results.push(SearchResult {
            title,
            url: url.to_string(),
            snippet,
        });
    }
    (results, false)
}
