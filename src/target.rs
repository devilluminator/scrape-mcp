use std::net::{Ipv4Addr, Ipv6Addr};

use url::{Host, Url};

use crate::error::{ScrapeError, ScrapeResult};
use crate::params::ExtractParams;

const SEARCH_URL: &str = "https://search.brave.com/search";

/// Turn tool parameters into the URL that should be fetched.
pub fn resolve(params: &ExtractParams, allow_private: bool) -> ScrapeResult<Url> {
    if params.search {
        search_url(params.keyword.as_deref())
    } else {
        page_url(params.url.as_deref(), allow_private)
    }
}

fn search_url(keyword: Option<&str>) -> ScrapeResult<Url> {
    let keyword = non_empty(keyword).ok_or(ScrapeError::MissingKeyword)?;
    let mut url = Url::parse(SEARCH_URL).expect("static URL is valid");
    url.query_pairs_mut()
        .append_pair("q", keyword)
        .append_pair("source", "web");
    Ok(url)
}
fn page_url(raw: Option<&str>, allow_private: bool) -> ScrapeResult<Url> {
    let raw = non_empty(raw).ok_or(ScrapeError::MissingUrl)?;
    let url = Url::parse(raw)?;

    if !matches!(url.scheme(), "http" | "https") {
        return Err(ScrapeError::UnsupportedScheme(url.scheme().to_owned()));
    }
    if !allow_private && !is_public_host(&url) {
        return Err(ScrapeError::ForbiddenHost(
            url.host_str().unwrap_or_default().to_owned(),
        ));
    }
    Ok(url)
}

fn non_empty(s: Option<&str>) -> Option<&str> {
    s.map(str::trim).filter(|s| !s.is_empty())
}

/// Basic SSRF guard: rejects localhost, `.local`/`.internal` names and
/// private/loopback/link-local IP literals. It does not resolve DNS, so a
/// public name pointing at a private IP is not caught.
pub fn is_public_host(url: &Url) -> bool {
    match url.host() {
        Some(Host::Domain(d)) => {
            let d = d.to_ascii_lowercase();
            !(d == "localhost"
                || d.ends_with(".localhost")
                || d.ends_with(".local")
                || d.ends_with(".internal"))
        }
        Some(Host::Ipv4(ip)) => !is_private_v4(ip),
        Some(Host::Ipv6(ip)) => !is_private_v6(ip),
        None => false,
    }
}

fn is_private_v4(ip: Ipv4Addr) -> bool {
    ip.is_private()
        || ip.is_loopback()
        || ip.is_link_local()
        || ip.is_unspecified()
        || ip.is_broadcast()
}

fn is_private_v6(ip: Ipv6Addr) -> bool {
    if let Some(v4) = ip.to_ipv4_mapped() {
        return is_private_v4(v4);
    }
    let first = ip.segments()[0];
    ip.is_loopback()
        || ip.is_unspecified()
        || (first & 0xfe00) == 0xfc00 // unique local fc00::/7
        || (first & 0xffc0) == 0xfe80 // link-local fe80::/10
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(search: bool, keyword: Option<&str>, url: Option<&str>) -> ExtractParams {
        ExtractParams {
            search,
            keyword: keyword.map(Into::into),
            url: url.map(Into::into),
        }
    }

    #[test]
    fn search_encodes_keyword() {
        let url = resolve(&params(true, Some("rust & mcp"), None), false).unwrap();
        assert_eq!(url.host_str(), Some("search.brave.com"));
        assert_eq!(url.query(), Some("q=rust+%26+mcp&source=web"));
    }
    #[test]
    fn rejects_bad_input() {
        assert!(resolve(&params(true, Some("  "), None), false).is_err());
        assert!(resolve(&params(false, None, None), false).is_err());
        assert!(resolve(&params(false, None, Some("ftp://example.com")), false).is_err());
        assert!(resolve(&params(false, None, Some("http://127.0.0.1:8000")), false).is_err());
        assert!(resolve(&params(false, None, Some("http://localhost")), false).is_err());
        assert!(resolve(&params(false, None, Some("http://127.0.0.1")), true).is_ok());
    }

    #[test]
    fn accepts_public_url() {
        assert!(resolve(&params(false, None, Some("https://example.com/x")), false).is_ok());
    }
}
