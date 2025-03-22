use std::sync::{Arc, Mutex};

use oauth2::basic::BasicClient;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::redis::RedisQueue;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: Arc<Mutex<RedisQueue>>,
    pub google_oauth: BasicClient,
    pub google_userinfo_url: String,
    pub csh_oauth: BasicClient,
    pub csh_userinfo_url: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<String>>,
}

impl From<String> for ApiError {
    fn from(value: String) -> Self {
        ApiError {
            error: Some(value),
            errors: None,
        }
    }
}

impl From<Vec<String>> for ApiError {
    fn from(value: Vec<String>) -> Self {
        ApiError {
            error: None,
            errors: Some(value),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimpleRiderChange {
    pub event_id: i32,
    pub car_id: i32,
    pub target_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct MultipleRiderChange {
    pub event_id: i32,
    pub car_id: i32,
    pub old_riders: Vec<String>,
    pub new_riders: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RedisJob {
    Join(SimpleRiderChange),
    Leave(SimpleRiderChange),
    RiderUpdate(MultipleRiderChange),
    Opening(SimpleRiderChange),
}
