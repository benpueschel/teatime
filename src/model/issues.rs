use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::model::user::User;

/// Represents an attachment.
/// Attachments are used in issues, pull requests, and releases.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Attachment {
    pub browser_download_url: String,
    pub created_at: String,
    pub download_count: i64,
    pub id: i64,
    pub name: String,
    pub size: i64,
    pub uuid: String,
}

/// Represents a label.
/// Labels are used in issues and pull requests.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Label {
    pub color: String,
    pub description: String,
    pub exclusive: bool,
    pub id: i64,
    pub is_archived: bool,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
/// Represents the state of an issue.
pub enum State {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "all")]
    #[default]
    All,
}
impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Open => write!(f, "open"),
            State::Closed => write!(f, "closed"),
            State::All => write!(f, "all"),
        }
    }
}

/// Represents an issue in a repository.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Issue {
    pub assets: Vec<Attachment>,
    pub assignee: Option<User>,
    pub assignees: Option<Vec<User>>,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub created_at: String,
    pub due_date: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub is_locked: bool,
    pub labels: Vec<Label>,
    pub number: i64,
    pub original_author: String,
    pub original_author_id: i64,
    pub pin_order: i64,
    pub r#ref: String,
    pub state: StateType,
    pub updated_at: String,
    pub title: String,
    pub url: String,
    pub user: User,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub assets: Vec<Attachment>,
    pub body: String,
    pub created_at: String,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub original_author: String,
    pub original_author_id: i64,
    pub pull_request_url: String,
    pub updated_at: String,
    pub user: User,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum StateType {
    #[default]
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    #[serde(rename = "issues")]
    Issues,
    #[serde(rename = "pulls")]
    Pulls,
}

impl Display for IssueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueType::Issues => write!(f, "issues"),
            IssueType::Pulls => write!(f, "pulls"),
        }
    }
}
