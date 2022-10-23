use crate::slack::SlackResult;

use super::{config::Config, Client, Message};

impl Client {
    pub fn new() -> Self {
        Client {
            config: Config::new(),
            client: reqwest::Client::new(),
        }
    }
    pub async fn send<'a>(&self, message: Message<'_>) {
        let app_token = self.config.app_token(message.app_token);
        let channel = self.config.channel(message.channel);

        let form = reqwest::multipart::Form::new()
            .text("channel", channel.to_string())
            .text("attachments", message.attachments());

        let res = self
            .client
            .post("https://slack.com/api/chat.postMessage")
            .header(
                reqwest::header::AUTHORIZATION,
                String::from("Bearer ") + app_token,
            )
            .multipart(form)
            .send()
            .await;

        let payload = res
            .expect("failed to get response")
            .text()
            .await
            .expect("failed to get payload");
        match serde_json::from_str::<SlackResult>(&payload) {
            Ok(result) => match result.ok {
                true => (),
                false => panic!("{}", payload),
            },
            Err(err) => {
                println!("{}", payload);
                panic!("{}", err)
            }
        };
    }
}
