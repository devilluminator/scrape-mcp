use std::sync::LazyLock;

use scraper::Selector;

fn sel(css: &str) -> Selector {
    Selector::parse(css).expect("valid static selector")
}

pub(super) static TITLE: LazyLock<Selector> = LazyLock::new(|| sel("title"));
pub(super) static H1: LazyLock<Selector> = LazyLock::new(|| sel("h1"));
pub(super) static P: LazyLock<Selector> = LazyLock::new(|| sel("p"));
pub(super) static A: LazyLock<Selector> = LazyLock::new(|| sel("a[href]"));

// Brave search results
pub(super) static RESULT: LazyLock<Selector> =
    LazyLock::new(|| sel(r#"div.snippet[data-type="web"]"#));
pub(super) static RESULT_TITLE: LazyLock<Selector> = LazyLock::new(|| sel(".search-snippet-title"));
pub(super) static RESULT_SNIPPET: LazyLock<Selector> =
    LazyLock::new(|| sel(".generic-snippet .content, .snippet-description"));
