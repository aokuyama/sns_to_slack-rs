use aws_lambda_events::event::sns::SnsEvent;
use lambda_runtime::{Error, LambdaEvent};
use serde::Serialize;
mod slack;

#[derive(Debug, Serialize)]
pub struct LambdaResult {}

pub async fn lambda_handler(event: LambdaEvent<serde_json::Value>) -> Result<LambdaResult, Error> {
    let sns: SnsEvent = serde_json::from_str(&event.payload.to_string()).unwrap();
    handle_sns(sns).await
}

async fn handle_sns(e: SnsEvent) -> Result<LambdaResult, Error> {
    let client = slack::Client::new();
    for record in e.records {
        let message = record.sns.message;
        let channel = match record.sns.message_attributes.get("channel") {
            Some(attr) => Some(&attr.value),
            None => None,
        };
        let app_token = match record.sns.message_attributes.get("app_token") {
            Some(attr) => Some(&attr.value),
            None => None,
        };
        client.send(&message, channel, app_token).await;
    }
    Ok(LambdaResult {})
}
