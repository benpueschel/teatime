use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::issues::Comment, Client};

#[derive(Debug, Clone, Builder, Serialize)]
#[build_it(into)]
pub struct ListAllCommentsBuilder {
    #[serde(skip)]
    #[build_it(skip)]
    owner: String,
    #[serde(skip)]
    #[build_it(skip)]
    repo: String,

    /// If provided, only comments updated since the provided time are returned.
    since: Option<String>,
    /// If provided, only comments updated before the provided time are returned.
    before: Option<String>,
    /// Page number of results to return (1-based).
    page: Option<i64>,
    /// Page size of results
    limit: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize)]
#[build_it(into)]
pub struct ListCommentsBuilder {
    #[serde(skip)]
    #[build_it(skip)]
    owner: String,
    #[serde(skip)]
    #[build_it(skip)]
    repo: String,
    #[serde(skip)]
    #[build_it(skip)]
    issue: i64,

    /// If provided, only comments updated since the provided time are returned.
    since: Option<String>,
    /// If provided, only comments updated before the provided time are returned.
    before: Option<String>,
    /// Page number of results to return (1-based).
    page: Option<i64>,
    /// Page size of results
    limit: Option<i64>,
}

impl ListAllCommentsBuilder {
    pub fn new(owner: &str, repo: &str) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            since: None,
            before: None,
            page: None,
            limit: None,
        }
    }

    /// Sends the request to list a repository's comments.
    pub async fn send(&self, client: &Client) -> Result<Vec<Comment>> {
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .get(format!("repos/{owner}/{repo}/issues/comments"))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

impl ListCommentsBuilder {
    pub fn new(owner: &str, repo: &str, issue: i64) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            issue,
            since: None,
            before: None,
            page: None,
            limit: None,
        }
    }

    /// Sends the request to list an issue's comments.
    pub async fn send(&self, client: &Client) -> Result<Vec<Comment>> {
        let owner = &self.owner;
        let repo = &self.repo;
        let issue = self.issue;
        let req = client
            .get(format!("repos/{owner}/{repo}/issues/{issue}/comments"))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
