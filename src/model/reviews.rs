use serde::{Deserialize, Serialize};
use super::{
    team::Team,
    user::User,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullReview {
    pub body: String,
    pub comments_count: i64,
    pub commit_id: String,
    pub dismissed: bool,
    pub html_url: String,
    pub id: i64,
    pub official: bool,
    pub pull_request_url: String,
    pub stale: bool,
    pub state: ReviewStateType,
    pub submitted_at: String,
    pub team: Option<Team>,
    pub updated_at: String,
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum ReviewStateType {
    #[default]
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "COMMENT")]
    Comment,
    #[serde(rename = "REQUEST_CHANGES")]
    RequestChanges,
    #[serde(rename = "REQUEST_REVIEW")]
    RequestReview,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
