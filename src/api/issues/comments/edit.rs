use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::issues::Comment, Client};

#[derive(Debug, Clone, Builder, Serialize)]
pub struct EditCommentBuilder {
    #[serde(skip)]
    #[build_it(skip)]
    owner: String,
    #[serde(skip)]
    #[build_it(skip)]
    repo: String,
    #[serde(skip)]
    #[build_it(skip)]
    comment: i64,

    /// The content of the comment.
    #[build_it(skip)]
    body: String,
    updated_at: Option<String>,
}

impl EditCommentBuilder {
    pub fn new(
        owner: impl ToString,
        repo: impl ToString,
        comment: i64,
        body: impl ToString,
    ) -> Self {
        Self {
            comment,
            owner: owner.to_string(),
            repo: repo.to_string(),
            body: body.to_string(),
            updated_at: None,
        }
    }

    /// Sends the request to edit a comment on an issue.
    /// NOTE: This is the only endpoint which returns an option. That's because the Gitea API
    /// decided - in their infinite wisdom - to sometimes return a 204 No Content status code
    /// when editing a comment, which means there's no response body to parse.
    pub async fn send(self, client: &Client) -> Result<Option<Comment>> {
        let owner = &self.owner;
        let repo = &self.repo;
        let comment = self.comment;
        let req = client
            .patch(format!("repos/{owner}/{repo}/issues/comments/{comment}"))
            .json(&self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
