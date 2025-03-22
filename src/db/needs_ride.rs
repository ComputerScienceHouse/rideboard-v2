use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, query_as, Executor, Postgres};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NeedsRide {
    pub user_id: String,
    pub event_id: i32,
}

impl NeedsRide {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        //let mut errs = Vec::new();
        Ok(())
    }
}

impl NeedsRide {
    pub async fn insert_new<'c, C>(user_id: String, event_id: i32, conn: C) -> Result<Self>
    where
        C: Executor<'c, Database = Postgres>,
    {
        query_as!(
            NeedsRide,
            r#"
            INSERT INTO needs_ride (user_id, event_id) VALUES ($1, $2) RETURNING *
            "#,
            user_id,
            event_id
        )
        .fetch_one(conn)
        .await
        .map_err(|err| anyhow!("Failed to Create Need Ride: {}", err))
    }

    pub async fn delete<'c, C>(user_id: String, event_id: i32, conn: C) -> Result<Option<PgRow>>
    where
        C: Executor<'c, Database = Postgres>,
    {
        sqlx::query!(
            "DELETE FROM needs_ride WHERE user_id = $1 AND event_id = $2",
            user_id,
            event_id
        )
        .fetch_optional(conn)
        .await
        .map_err(|err| anyhow!("Failed to Delete Need Ride: {}", err))
    }

    pub async fn select_all<'c, C>(event_id: i32, conn: C) -> Result<Vec<Self>>
    where
        C: Executor<'c, Database = Postgres>,
    {
        query_as!(
            NeedsRide,
            r#"
            SELECT user_id AS "user_id!", event_id AS "event_id!"
            FROM needs_ride
            WHERE event_id = $1
            "#,
            event_id
        )
        .fetch_all(conn)
        .await
        .map_err(|err| anyhow!("Failed to get needs_ride: {}", err))
    }
}
