use envy::from_env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub slack_app_token: Option<String>,
    pub slack_default_channel: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        from_env::<Self>().unwrap()
    }
    pub fn app_token<'a>(&'a self, app_token: Option<&'a String>) -> &str {
        match app_token {
            Some(x) => x,
            None => match &self.slack_app_token {
                Some(y) => &y,
                None => panic!("undefined app token"),
            },
        }
    }
    pub fn channel<'a>(&'a self, channel: Option<&'a String>) -> &str {
        match channel {
            Some(x) => x,
            None => match &self.slack_default_channel {
                Some(y) => &y,
                None => panic!("undefined destination channel"),
            },
        }
    }
}
