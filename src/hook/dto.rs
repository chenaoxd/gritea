use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{repo::Repository, user::User};

/// Options when create a hook
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateHookOption {
    #[serde(rename = "type")]
    pub type_: String,
    /// NOTE: url, content_type is required in config
    pub config: HashMap<String, String>,
    pub events: Vec<String>,
    pub branch_filter: String,
    pub active: bool,
}

/// A hook is a web hook when one repository changed
#[derive(Debug, Deserialize, Serialize)]
pub struct Hook {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: String,
    pub config: HashMap<String, String>,
    pub events: Vec<String>,
    pub active: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PayloadUser {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PayloadCommit {
    pub id: String,
    pub message: String,
    pub url: String,
    pub author: PayloadUser,
    pub committer: PayloadUser,
    pub verification: Option<PayloadCommitVerification>,
    pub timestamp: DateTime<Utc>,
    pub added: Option<Vec<String>>,
    pub removed: Option<Vec<String>>,
    pub modified: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PayloadCommitVerification {
    pub verified: bool,
    pub reason: String,
    pub signature: String,
    pub signer: PayloadUser,
    pub payload: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PushPayload {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub before: String,
    pub after: String,
    pub compare_url: String,
    pub commits: Vec<PayloadCommit>,
    pub head_commit: PayloadCommit,
    pub repository: Repository,
    pub pusher: User,
    pub sender: User,
}
