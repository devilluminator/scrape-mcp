//! A small MCP server exposing one tool, `web_extract`, that fetches a page
//! (or a Brave search result page) and returns its structured content.

pub mod config;
pub mod error;
pub mod extract;
pub mod fetch;
pub mod page;
pub mod params;
pub mod serve;
pub mod server;
pub mod target;
