use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::user::User;

/// Permission represents a set of permissions
#[derive(Debug, Deserialize, Serialize)]
pub struct Permission {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

/// InternalTracker represents settings for internal tracker
#[derive(Debug, Deserialize, Serialize)]
pub struct InternalTracker {
    enable_time_tracker: bool,
    allow_only_contributors_to_track_time: bool,
    enable_issue_dependencies: bool,
}

/// Repository represents a repository
#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    pub id: i64,
    pub owner: User,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub empty: bool,
    pub private: bool,
    pub fork: bool,
    pub template: bool,
    pub parent: Option<Box<Repository>>,
    pub mirror: bool,
    pub size: i32,
    pub html_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub original_url: String,
    pub website: String,
    pub stars_count: i32,
    pub forks_count: i32,
    pub watchers_count: i32,
    pub open_issues_count: i32,
    pub open_pr_counter: i32,
    pub release_counter: i32,
    pub default_branch: String,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub permissions: Permission,
    pub has_issues: bool,
    pub internal_tracker: InternalTracker,
    pub has_wiki: bool,
    pub has_pull_requests: bool,
    pub has_projects: bool,
    pub ignore_whitespace_conflicts: bool,
    pub allow_merge_commits: bool,
    pub allow_rebase: bool,
    pub allow_rebase_explicit: bool,
    pub allow_squash_merge: bool,
    pub default_merge_style: String,
    pub avatar_url: String,
    pub internal: bool,
    pub mirror_interval: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitStatusState {
    Pending,
    Success,
    Error,
    Failure,
    Warning,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitStatus {
    pub id: i64,
    pub status: CommitStatusState,
    pub target_url: String,
    pub description: String,
    pub url: String,
    pub context: String,
    pub creator: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateStatusOption {
    pub state: CommitStatusState,
    pub target_url: String,
    pub description: String,
    pub context: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_status_state_serialize() {
        let objs = vec![
            CommitStatusState::Pending,
            CommitStatusState::Success,
            CommitStatusState::Error,
            CommitStatusState::Failure,
            CommitStatusState::Warning,
        ];
        let reprs = vec![
            "\"pending\"",
            "\"success\"",
            "\"error\"",
            "\"failure\"",
            "\"warning\"",
        ];

        for (obj, repr) in objs.iter().zip(reprs.iter()) {
            let res_repr = serde_json::to_string(obj).unwrap();
            assert_eq!(&res_repr, repr);
        }
    }
}
