mod slack;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let message = &args[1];

    let client = slack::Client::new();
    client.send(message, None, None).await;
}
