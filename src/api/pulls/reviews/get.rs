use build_it::Builder;
use serde::Serialize;

use crate::{
    error::Result,
    model::{
        reviews::PullReview,
    },
    Client,
};

#[derive(Debug, Clone, Serialize, Builder)]
#[build_it(into)]
pub struct GetReviewsBuilder {
    #[serde(skip)]
    #[skip]
    owner: String,
    #[serde(skip)]
    #[skip]
    repo: String,
    /// Index of the pull request
    #[serde(skip)]
    #[skip]
    index: i64,

    /// Page number of results to return (1-based)
    page: Option<i64>,
    /// Page size of results
    limit: Option<i64>,
}

impl GetReviewsBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, index: i64) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            index,
            page: None,
            limit: None,
        }
    }
    /// Sends the request to list all reviews for a pull request.
    pub async fn send(&self, client: &Client) -> Result<Vec<PullReview>> {
        let owner = &self.owner;
        let repo = &self.repo;
        let index = &self.index;
        let req = client
            .get(format!("repos/{owner}/{repo}/pulls/{index}/reviews"))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
