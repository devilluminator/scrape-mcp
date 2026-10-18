use std::env;
use std::time::Duration;

/// Caps that keep tool output usable as LLM context.
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_paragraphs: usize,
    pub max_links: usize,
    pub max_chars: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_paragraphs: 200,
            max_links: 200,
            max_chars: 2_000,
        }
    }
}

/// Runtime configuration. Override via environment variables:
/// `MCP_BIND`, `MCP_MOUNT`, `MCP_ALLOW_PRIVATE` (`1`/`true`).
#[derive(Debug, Clone)]
pub struct Config {
    pub bind: String,
    pub mount: String,
    pub timeout: Duration,
    pub max_body_bytes: usize,
    pub allow_private_hosts: bool,
    pub limits: Limits,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1:8000".into(),
            mount: "/mcp".into(),
            timeout: Duration::from_secs(30),
            max_body_bytes: 5 * 1024 * 1024,
            allow_private_hosts: false,
            limits: Limits::default(),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let mut cfg = Self::default();
        if let Ok(v) = env::var("MCP_BIND") {
            cfg.bind = v;
        }
        if let Ok(v) = env::var("MCP_MOUNT") {
            cfg.mount = v;
        }
        if let Ok(v) = env::var("MCP_ALLOW_PRIVATE") {
            cfg.allow_private_hosts = matches!(v.as_str(), "1" | "true" | "TRUE");
        }
        cfg
    }
}
