use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TODO: parse datetime
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub active: bool,
    pub avatar_url: String,
    pub created: DateTime<Utc>,
    pub description: String,
    pub email: String,
    pub followers_count: i64,
    pub following_count: i64,
    pub full_name: String,
    pub id: i64,
    pub is_admin: bool,
    pub language: String,
    pub last_login: DateTime<Utc>,
    pub location: String,
    pub login: String,
    pub prohibit_login: bool,
    pub restricted: bool,
    pub starred_repos_count: i64,
    pub visibility: String,
    pub website: String,
}
