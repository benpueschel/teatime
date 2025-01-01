use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::{
    orgs::Organization,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub can_create_org_repo: bool,
    pub description: String,
    pub id: i64,
    pub includes_all_repositories: bool,
    pub name: String,
    pub organization: Option<Organization>,
    pub permission: Permission,
    pub units: Vec<String>,
    pub units_map: HashMap<String, String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "owner")]
    Owner,
}
