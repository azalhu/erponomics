use std::env;

use anyhow::Context;

const SERVER_PORT_KEY: &str = "ERP_MNF_SERVER_PORT";

const DATABASE_URL_KEY: &str = "ERP_MNF_DB_URL";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let server_port = load_env(SERVER_PORT_KEY)?.parse()?;
        let database_url = load_env(DATABASE_URL_KEY)?;

        Ok(Self {
            server_port,
            database_url,
        })
    }
}

fn load_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {key}"))
}
