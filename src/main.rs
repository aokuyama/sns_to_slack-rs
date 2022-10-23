mod slack;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let message = &args[1];

    let client = slack::Client::new();
    let title = "test message".to_owned();
    let msg = slack::Message::new(message, Some(title), None, None);
    client.send(msg).await;
}
