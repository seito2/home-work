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
