use std::env;

use infrastructure::repository::find_homeworks;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use sqlx::MySqlPool;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request, pool: MySqlPool) -> Result<Response<Body>, Error> {
    // Return something that implements IntoResponse.

    let homeworks = find_homeworks(pool).await?;

    let message = serde_json::to_string(&homeworks)?;

    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(|e| async {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
        let pool = MySqlPool::connect(url.as_str()).await?;

        util::in_out_logger(e, pool, function_handler).await
    }))
    .await
}
