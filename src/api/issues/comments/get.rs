use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::issues::Comment, Client};

#[derive(Debug, Clone)]
pub struct GetCommentBuilder {
    owner: String,
    repo: String,
    comment: i64,
}

impl GetCommentBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, comment: i64) -> Self {
        Self {
            comment,
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    /// Sends the request to get a comment on an issue.
    pub async fn send(self, client: &Client) -> Result<Comment> {
        let owner = &self.owner;
        let repo = &self.repo;
        let comment = self.comment;
        let req = client
            .get(format!("repos/{owner}/{repo}/issues/comments/{comment}"))
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
