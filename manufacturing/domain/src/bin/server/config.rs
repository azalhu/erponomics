use std::env;

use anyhow::Context;

const SERVER_PORT_KEY: &str = "ERP_MNF_DOM_SERVER_PORT";

const DB_URL_KEY: &str = "ERP_MNF_DB_SERVER_URL";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_port: u16,
    pub db_server_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let server_port = load_env(SERVER_PORT_KEY)?.parse()?;
        let db_server_url = load_env(DB_URL_KEY)?;

        Ok(Self {
            server_port,
            db_server_url,
        })
    }
}

fn load_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {key}"))
}
