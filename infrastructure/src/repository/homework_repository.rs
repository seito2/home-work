use std::{future::Future, thread::sleep, time};

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

pub async fn create_homework(
    pool: MySqlPool,
    display_name: String,
    description: String,
) -> Result<HomeWork, Error> {
    sqlx::query!(
        r#"
            INSERT INTO homeworks (display_name, description)
            VALUES (?, ?)
        "#,
        display_name,
        description
    )
    .execute(&pool)
    .await?;

    // すぐにSELECTすると、INSERTしたデータが取得できないので、少し待つ
    sleep(time::Duration::from_millis(10));

    let created: QuerableHomeWork = sqlx::query_as!(
        QuerableHomeWork,
        r#"
            SELECT 
                *
            FROM 
                homeworks
            WHERE
                id = (SELECT LAST_INSERT_ID())
        "#
    )
    .fetch_one(&pool)
    .await?;

    println!("created: {:?}", created.id);

    Ok(created.into())
}

pub async fn update_homework(
    pool: MySqlPool,
    id: i32,
    display_name: String,
    description: String,
) -> Result<HomeWork, Error> {
    sqlx::query!(
        r#"
            UPDATE homeworks
            SET
                display_name = ?,
                description = ?
            WHERE
                id = ?
        "#,
        display_name,
        description,
        id
    )
    .execute(&pool)
    .await?;

    // すぐにSELECTすると、INSERTしたデータが取得できないので、少し待つ
    sleep(time::Duration::from_millis(10));

    let updated: QuerableHomeWork = sqlx::query_as!(
        QuerableHomeWork,
        r#"
            SELECT 
                *
            FROM 
                homeworks
            WHERE
                id = ?
        "#,
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok(updated.into())
}
