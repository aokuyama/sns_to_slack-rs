mod client;
mod config;
use self::config::Config;
use serde::Deserialize;

pub struct Client {
    config: Config,
    client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
struct SlackResult {
    ok: bool,
}
