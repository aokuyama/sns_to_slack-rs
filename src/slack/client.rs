use super::{config::{Config}, Client};


impl Client {
    pub fn new() -> Self {
        Client{
            config: Config::new(),
            client: reqwest::Client::new(),
        }
    }
    pub async fn send<'a>(&self, message: &'a str, channel: Option<&'a String>, app_token: Option<&String>) {
        let app_token = self.config.app_token(app_token);
        let channel = self.config.channel(channel);

        let form = reqwest::multipart::Form::new()
        .text("channel", channel.to_string())
        .text("text", message.to_string());

        let res = self.client.post("https://slack.com/api/chat.postMessage")
        .header(reqwest::header::AUTHORIZATION, String::from("Bearer ") + app_token)
        .multipart(form)
        .send()
        .await;

        let payload = res.expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");
        println!("{}", payload);
    }
}
