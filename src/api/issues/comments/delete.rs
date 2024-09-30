use crate::{error::Result, Client};

#[derive(Debug, Clone)]
pub struct DeleteCommentBuilder {
    owner: String,
    repo: String,
    comment: i64,
}

impl DeleteCommentBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, comment: i64) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            comment,
        }
    }

    /// Sends the request to delete a comment.
    pub async fn send(&self, client: &Client) -> Result<()> {
        let req = client
            .delete(format!(
                "repos/{}/{}/issues/comments/{}",
                self.owner, self.repo, self.comment
            ))
            .build()?;
        let _ = client.make_request(req).await?;
        Ok(())
    }
}
