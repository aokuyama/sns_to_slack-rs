mod client;
mod config;
use self::config::Config;

pub struct Client {
    config: Config,
    client: reqwest::Client,
}
