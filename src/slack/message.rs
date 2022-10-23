use super::{Body, Message};

impl Message<'_> {
    pub fn new<'a>(
        body: &'a str,
        title: Option<String>,
        channel: Option<&'a String>,
        app_token: Option<&'a String>,
    ) -> Message<'a> {
        let body = Body { text: body, title };
        Message {
            body,
            channel,
            app_token,
        }
    }
    pub fn attachments(&self) -> String {
        let attachments: [&Body; 1] = [&self.body];
        serde_json::to_string(&attachments).unwrap()
    }
}
