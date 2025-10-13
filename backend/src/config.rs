use anyhow::Context;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};
use std::{collections::HashMap, fs, net::SocketAddr, path::Path};

/// Wrapper type for [`mime::Mime`] since it doesn't implement [`Deserialize`]
#[serde_as]
#[derive(Deserialize, Deref, DerefMut, PartialEq, Eq)]
struct Mime(#[serde_as(as = "DisplayFromStr")] mime::Mime);

// #[serde_as]
#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: SocketAddr,
    pub database_url: String,
    default_mime: Mime,
}
impl ServerConfig {
    pub fn default_mime(&self) -> &mime::Mime {
        &self.default_mime.0
    }
}

#[derive(Deserialize)]
pub struct ContentRules {
    allowed_mimes: Vec<Mime>,
}
impl ContentRules {
    pub fn allows_mime(&self, mime: mime::Mime) -> bool {
        self.allowed_mimes.contains(&Mime(mime))
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    #[serde(alias = "content")]
    pub content_rules: HashMap<String, ContentRules>,
}
impl Config {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        fs::read_to_string(path)
            .context("Cannot find config")
            .and_then(|config| toml::from_str(&config).context("Cannot parse config"))
    }
    pub fn database_url(&self) -> &str {
        &self.server.database_url
    }
}
