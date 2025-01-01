use serde::{Deserialize, Serialize};

/// Represents a Gitea organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub visibility: Visibility,
    pub repo_admin_change_team_access: bool,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

/// Represents the visibility of an organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Limited,
    Private,
}
