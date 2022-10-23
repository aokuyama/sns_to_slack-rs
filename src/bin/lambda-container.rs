use sns_to_slack::lambda_handler;

use lambda_runtime::Error as LambdaError;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let handler = lambda_runtime::service_fn(lambda_handler);
    let r = lambda_runtime::run(handler).await?;
    Ok(r)
}
