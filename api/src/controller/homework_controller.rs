use infrastructure::repository::homework_repository;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::MySqlPool;
use validator::Validate;

pub async fn show_homeworks(_event: Request, pool: MySqlPool) -> Result<Response<Body>, Error> {
    let homeworks = homework_repository::find_homeworks(pool).await?;

    let message = serde_json::to_string(&homeworks)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[derive(Validate, Deserialize, Serialize)]
pub struct CreateHomeworkRequest {
    #[validate(length(min = 1, max = 255))]
    pub display_name: String,
    #[validate(length(min = 1, max = 255))]
    pub description: String,
}

pub async fn create_homework(event: Request, pool: MySqlPool) -> Result<Response<Body>, Error> {
    let request_validation = util::validate_body::<CreateHomeworkRequest>(event.body());

    if request_validation.is_err() {
        let err = request_validation.err().unwrap();
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(json!({ "message": err.to_string() }).to_string().into())
            .expect("failed to render response"));
    }

    let body = request_validation.ok().unwrap();

    let homework =
        homework_repository::create_homework(pool, body.display_name, body.description).await?;

    let message = serde_json::to_string(&homework)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateHomeworkRequest {
    #[validate(range(min = 1))]
    pub id: i32,
    #[validate(length(min = 1, max = 255))]
    pub display_name: String,
    #[validate(length(min = 1, max = 255))]
    pub description: String,
}

pub async fn update_homework(event: Request, pool: MySqlPool) -> Result<Response<Body>, Error> {
    let request_validation = util::validate_body::<UpdateHomeworkRequest>(event.body());

    if request_validation.is_err() {
        let err = request_validation.err().unwrap();
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(json!({ "message": err.to_string() }).to_string().into())
            .expect("failed to render response"));
    }

    let body = request_validation.ok().unwrap();

    let homework =
        homework_repository::update_homework(pool, body.id, body.display_name, body.description)
            .await?;

    let message = serde_json::to_string(&homework)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}
