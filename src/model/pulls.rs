use serde::{Deserialize, Serialize};

use super::{
    issues::{Label, StateType},
    repos::Repository,
    user::User,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub additions: i64,
    pub allow_maintainer_edit: bool,
    pub assignees: Option<Vec<User>>,
    pub base: PrBranchInfo,
    pub body: String,
    pub changed_files: i64,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub created_at: String,
    pub deletions: i64,
    pub diff_url: String,
    pub draft: bool,
    pub due_date: Option<String>,
    pub head: PrBranchInfo,
    pub html_url: String,
    pub id: i64,
    pub is_locked: bool,
    pub labels: Vec<Label>,
    pub merge_base: String,
    pub merge_commit_sha: Option<String>,
    pub mergeable: bool,
    pub merged: bool,
    pub merged_at: Option<String>,
    pub merged_by: Option<User>,
    // TODO: pub milestone: Option<Milestone>,
    pub number: i64,
    pub patch_url: String,
    pub pin_order: i64,
    pub requested_reviewers: Option<Vec<Option<User>>>,
    /// Number of review comments made on the diff of a PR review (not including comments on commits or issues in a PR)
    // Gitea has this marked as `omitempty` so it's omitted when zero.
    #[serde(default)]
    pub review_comments: i64,
    pub state: StateType,
    pub title: String,
    // TODO: Make this a DateTime<Utc>
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrBranchInfo {
    pub label: String,
    pub r#ref: String,
    pub repo: Repository,
    pub repo_id: i64,
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    Oldest,
    RecentUpdate,
    LeastUpdate,
    MostComment,
    LeastComment,
    Priority,
}
