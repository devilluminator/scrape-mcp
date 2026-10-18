use std::collections::HashSet;

use scraper::Html;
use url::Url;

use super::selectors;
use super::text::text_of;
use crate::config::Limits;
use crate::page::Link;

/// Collect de-duplicated absolute http(s) links. The bool is `true` when the
/// `max_links` cap cut the list short.
pub(super) fn collect(doc: &Html, base: &Url, limits: &Limits) -> (Vec<Link>, bool) {
    let mut seen = HashSet::new();
    let mut links = Vec::new();

    for el in doc.select(&selectors::A) {
        let Some(url) = el.value().attr("href").and_then(|h| resolve(base, h)) else {
            continue;
        };
        if !seen.insert(url.clone()) {
            continue;
        }
        if links.len() >= limits.max_links {
            return (links, true);
        }
        links.push(Link {
            text: text_of(el, limits.max_chars),
            href: url.to_string(),
        });
    }
    (links, false)
}

/// Resolve `href` against `base`; drops anchors, `mailto:`, `javascript:` etc.
/// and strips fragments so `/a#x` and `/a#y` de-duplicate.
pub(super) fn resolve(base: &Url, href: &str) -> Option<Url> {
    let href = href.trim();
    if href.is_empty() || href.starts_with('#') {
        return None;
    }
    let mut url = base.join(href).ok()?;
    if !matches!(url.scheme(), "http" | "https") {
        return None;
    }
    url.set_fragment(None);
    Some(url)
}
