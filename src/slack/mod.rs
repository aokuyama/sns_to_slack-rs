mod client;
mod config;
mod message;
use self::config::Config;
use serde::{Deserialize, Serialize};

pub struct Client {
    config: Config,
    client: reqwest::Client,
}

pub struct Message<'a> {
    body: Body<'a>,
    channel: Option<&'a String>,
    app_token: Option<&'a String>,
}
#[derive(Serialize)]
struct Body<'a> {
    text: &'a str,
    title: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SlackResult {
    ok: bool,
}
