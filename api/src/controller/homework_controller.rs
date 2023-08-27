use infrastructure::repository::find_homeworks;
use lambda_http::{Body, Error, Request, Response};
use sqlx::MySqlPool;

pub async fn show_homeworks(event: Request, pool: MySqlPool) -> Result<Response<Body>, Error> {
    // Return something that implements IntoResponse.

    print!("event: {:?}", event);

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
