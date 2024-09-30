use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::issues::Comment, Client};

#[derive(Debug, Clone, Builder, Serialize)]
pub struct CreateCommentBuilder {
    #[serde(skip)]
    #[build_it(skip)]
    owner: String,
    #[serde(skip)]
    #[build_it(skip)]
    repo: String,
    #[serde(skip)]
    #[build_it(skip)]
    issue: i64,

    /// The content of the comment.
    #[build_it(skip)]
    body: String,
    updated_at: Option<String>,
}

impl CreateCommentBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, issue: i64, body: impl ToString) -> Self {
        Self {
            issue,
            owner: owner.to_string(),
            repo: repo.to_string(),
            body: body.to_string(),
            updated_at: None,
        }
    }

    /// Sends the request to create a comment on an issue.
    pub async fn send(self, client: &Client) -> Result<Comment> {
        let owner = &self.owner;
        let repo = &self.repo;
        let issue = self.issue;
        let req = client
            .post(format!("repos/{owner}/{repo}/issues/{issue}/comments"))
            .json(&self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
