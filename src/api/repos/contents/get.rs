use build_it::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{model::repos::Entry, Result};

#[derive(Debug, Serialize, Deserialize, Builder)]
#[build_it(into)]
pub struct GetContentsRepoBuilder {
    /// The owner of the repository.
    #[skip]
    #[serde(skip)]
    owner: String,
    /// The name of the repository.
    #[skip]
    #[serde(skip)]
    repo: String,
    /// The entry filepath can be a directory or file.
    #[skip]
    #[serde(skip)]
    filepath: String,
    /// The name of the commit/branch/tag. Default the repository’s default branch (usually master)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[build_it(rename = "refs")]
    r#ref: Option<String>,
}

impl GetContentsRepoBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, filepath: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            filepath: filepath.to_string(),
            r#ref: None,
        }
    }

    /// Send the request to fetch given repository's file path contents.
    pub async fn send(&self, client: &crate::Client) -> Result<Vec<Entry>> {
        let owner = &self.owner;
        let repo = &self.repo;
        let filepath = &self.filepath;
        let req = client
            .get(format!("repos/{owner}/{repo}/contents/{filepath}"))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;

        let res_body: Value = res.json().await?;

        // Response can either be Entry or Entry[], put in Vec<> to handle both cases
        let entries = serde_json::from_value::<Vec<Entry>>(res_body.clone()).or_else(|_| {
            serde_json::from_value(res_body.clone()).map(|single_entry| vec![single_entry])
        })?;

        Ok(entries)
    }
}
