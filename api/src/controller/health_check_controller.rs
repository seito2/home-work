use lambda_http::{Body, Error, Request, Response};
use sqlx::MySqlPool;

pub async fn not_found(_event: Request, _pool: MySqlPool) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(404)
        .body("It's Not Found!! hahaha.".into())
        .map_err(Box::new)?;
    Ok(resp)
}
