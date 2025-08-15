use build_it::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    model::repos::{CommitDateOptions, EntryMutation, Identity},
    Result,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
#[build_it(into)]
pub struct DeleteFileRepoBuilder {
    #[skip]
    #[serde(skip)]
    owner: String,

    #[skip]
    #[serde(skip)]
    repo: String,

    #[skip]
    #[serde(skip)]
    filepath: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<Identity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    committer: Option<Identity>,

    dates: Option<CommitDateOptions>,

    message: Option<String>,

    new_branch: Option<String>,

    #[skip]
    sha: String,

    signoff: Option<bool>,
}

impl DeleteFileRepoBuilder {
    pub fn new(
        owner: impl ToString,
        repo: impl ToString,
        filepath: impl ToString,
        sha: impl ToString,
    ) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            filepath: filepath.to_string(),
            author: None,
            branch: None,
            committer: None,
            dates: None,
            message: None,
            new_branch: None,
            sha: sha.to_string(),
            signoff: None,
        }
    }

    /// Send the request to delete file from given repository.
    pub async fn send(&self, client: &crate::Client) -> Result<EntryMutation> {
        let owner = &self.owner;
        let repo = &self.repo;
        let filepath = &self.filepath;

        let req = client
            .delete(format!("repos/{owner}/{repo}/contents/{filepath}"))
            .json(self)
            .build()?;

        let res = client.make_request(req).await?;

        client.parse_response(res).await
    }
}
