use std::future::Future;

use lambda_http::Error;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

struct QuerableHomeWork {
    id: i32,
    display_name: String,
    description: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct HomeWork {
    id: i32,
    display_name: String,
    description: String,
    created_at: String,
    updated_at: String,
}

impl From<QuerableHomeWork> for HomeWork {
    fn from(qhw: QuerableHomeWork) -> Self {
        HomeWork {
            id: qhw.id,
            display_name: qhw.display_name,
            description: qhw.description,
            created_at: qhw.created_at.to_string(),
            updated_at: qhw.updated_at.to_string(),
        }
    }
}

pub fn find_homeworks(pool: MySqlPool) -> impl Future<Output = Result<Vec<HomeWork>, Error>> {
    async move {
        let homeworks = sqlx::query_as!(
            QuerableHomeWork,
            r#"
            SELECT * FROM homeworks
            "#
        )
        .fetch_all(&pool)
        .await?;

        Ok(homeworks.into_iter().map(|hw| hw.into()).collect())
    }
}
