mod links;
mod search;
mod selectors;
mod text;

use scraper::{Html, Selector};
use url::Url;

use crate::config::Limits;
use crate::page::Page;

use text::text_of;

/// Parse `html` and pull out its content. Brave search pages come back as
/// structured `results`; any other page as title, `<h1>`, paragraphs and links.
pub fn extract_page(html: &str, base: &Url, limits: &Limits) -> Page {
    let doc = Html::parse_document(html);

    let first_text = |sel: &Selector| {
        doc.select(sel)
            .next()
            .map(|el| text_of(el, limits.max_chars))
            .filter(|s| !s.is_empty())
    };
    let title = first_text(&selectors::TITLE);
    let header = first_text(&selectors::H1);

    let (results, results_truncated) = search::collect(&doc, base, limits);
    if !results.is_empty() {
        return Page {
            source_url: base.to_string(),
            title,
            header,
            results,
            paragraphs: Vec::new(),
            links: Vec::new(),
            truncated: results_truncated,
        };
    }

    // Take one extra so we can tell "exactly at the cap" from "over the cap".
    let mut paragraphs: Vec<String> = doc
        .select(&selectors::P)
        .map(|el| text_of(el, limits.max_chars))
        .filter(|s| !s.is_empty())
        .take(limits.max_paragraphs + 1)
        .collect();
    let paragraphs_truncated = paragraphs.len() > limits.max_paragraphs;
    paragraphs.truncate(limits.max_paragraphs);

    let (links, links_truncated) = links::collect(&doc, base, limits);

    Page {
        source_url: base.to_string(),
        title,
        header,
        results: Vec::new(),
        paragraphs,
        links,
        truncated: paragraphs_truncated || links_truncated,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_structure() {
        let html = r##"
            <html><head><title>T</title></head><body>
              <h1> Hello   world </h1>
              <p>One</p><p>   </p><p>Two <b>bold</b></p>
              <a href="/a#x">A</a><a href="/a#y">dup</a>
              <a href="#top">skip</a><a href="mailto:a@b.c">mail</a>
            </body></html>"##;
        let base = Url::parse("https://example.com/dir/").unwrap();
        let page = extract_page(html, &base, &Limits::default());

        assert_eq!(page.title.as_deref(), Some("T"));
        assert_eq!(page.header.as_deref(), Some("Hello world"));
        assert_eq!(page.paragraphs, ["One", "Two bold"]);
        assert_eq!(page.links.len(), 1);
        assert_eq!(page.links[0].href, "https://example.com/a");
        assert!(page.results.is_empty());
        assert!(!page.truncated);
    }

    #[test]
    fn flags_truncation() {
        let limits = Limits {
            max_paragraphs: 1,
            ..Limits::default()
        };
        let base = Url::parse("https://example.com/").unwrap();
        let page = extract_page("<p>a</p><p>b</p>", &base, &limits);
        assert_eq!(page.paragraphs.len(), 1);
        assert!(page.truncated);
    }

    #[test]
    fn extracts_brave_results() {
        let html = r##"
            <html><head><title>Rust - Brave Search</title></head><body>
            <a href="/settings">Settings</a>
            <div class="snippet svelte-x" data-pos="1" data-type="web">
                <div class="result-content">
                <a href="https://rust-lang.org/en-US/" class="l1">
                    <cite class="snippet-url">rust-lang.org</cite>
                    <div class="title search-snippet-title">Rust Programming Language</div>
                </a>
                <div class="generic-snippet"><div class="content"><!---->A language
                    empowering everyone.</div></div>
                </div>
            </div>
            <div class="snippet" data-pos="2" data-type="news">
                <a href="https://news.example/">Not a web result</a>
            </div>
            </body></html>"##;
        let base = Url::parse("https://search.brave.com/search?q=Rust").unwrap();
        let page = extract_page(html, &base, &Limits::default());

        assert_eq!(page.results.len(), 1);
        assert_eq!(page.results[0].title, "Rust Programming Language");
        assert_eq!(page.results[0].url, "https://rust-lang.org/en-US/");
        assert_eq!(page.results[0].snippet, "A language empowering everyone.");
        assert!(page.links.is_empty() && page.paragraphs.is_empty());
    }
}
