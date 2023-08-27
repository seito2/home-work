use lambda_http::{Body, Error, Request, Response};
use sqlx::MySqlPool;

pub async fn in_out_logger<F, Res>(
    event: Request,
    pool: MySqlPool,
    function: F,
) -> Result<Response<Body>, Error>
where
    F: FnOnce(Request, MySqlPool) -> Res,
    Res: std::future::Future<Output = Result<Response<Body>, Error>>,
{
    tracing::info!("Event: {:?}", event);

    let res = function(event, pool).await;

    tracing::info!("Response: {:?}", res);

    res
}

pub fn validate_body<T>(body: &Body) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let res = match body {
        Body::Text(body) => serde_json::from_str::<T>(&body),
        Body::Binary(body) => serde_json::from_slice::<T>(&body),
        _ => return Err("Empty request body".to_string()),
    };

    res.map_err(|err| Error::from(err).to_string())
}
