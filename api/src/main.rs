use std::env;

use controller::{health_check_controller, homework_controller};
use lambda_http::{run, service_fn, Error, Request};
use sqlx::MySqlPool;

mod controller;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(|e: Request| async {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
        let pool = MySqlPool::connect(url.as_str()).await?;

        let route = e
            .uri()
            .path_and_query()
            .map(|e| e.as_str().replace("/?path=", ""));

        if route.is_none() {
            return util::in_out_logger(e, pool, health_check_controller::not_found).await;
        }

        let route = route.unwrap();

        println!("route: {}", route);
        println!("query: {:?}", e.uri().path_and_query());

        match route.as_str() {
            "/homeworks" => {
                return util::in_out_logger(e, pool, homework_controller::show_homeworks).await
            }
            "/homeworks/create" => {
                return util::in_out_logger(e, pool, homework_controller::create_homework).await
            }
            "/homeworks/update" => {
                return util::in_out_logger(e, pool, homework_controller::update_homework).await
            }
            _ => return util::in_out_logger(e, pool, health_check_controller::not_found).await,
        }
    }))
    .await
}
