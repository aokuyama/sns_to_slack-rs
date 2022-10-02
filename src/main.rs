#[tokio::main]
async fn main() {
    let text = "test message.";
    let token = "xoxb-xxxxxx";
    let channel = "C0XXXXXX";

    let form = reqwest::multipart::Form::new()
    .text("channel", channel)
    .text("text", text);

    let client = reqwest::Client::new();
    let res = client.post("https://slack.com/api/chat.postMessage")
    .header(reqwest::header::AUTHORIZATION, String::from("Bearer ") + token)
    .multipart(form)
    .send()
    .await;

    let payload = res.expect("failed to get response")
    .text()
    .await
    .expect("failed to get payload");
    println!("{}", payload);
}
