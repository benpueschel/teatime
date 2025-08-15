use build_it::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    model::repos::{CommitDateOptions, EntryMutation, Identity},
    Result,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
#[build_it(into)]
pub struct UpdateFileRepoBuilder {
    /// The owner of the repository.
    #[skip]
    #[serde(skip)]
    owner: String,
    /// The name of the repository.
    #[skip]
    #[serde(skip)]
    repo: String,
    /// Path of the file to update.
    #[skip]
    #[serde(skip)]
    filepath: String,
    /// Identity for a person's identity like an author or committer.
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<Identity>,
    /// Branch (optional) to base this file from. if not given, the default branch is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<String>,
    // Identity for a person's identity like an author or committer
    #[serde(skip_serializing_if = "Option::is_none")]
    committer: Option<Identity>,
    /// File content must be base64 encoded
    #[skip]
    content: String,
    /// CommitDateOptions store dates for GIT_AUTHOR_DATE and GIT_COMMITTER_DATE
    dates: Option<CommitDateOptions>,
    /// from_path (optional) is the path of the original file which will be moved/renamed to the path in the URL
    from_path: Option<String>,
    /// message (optional) for the commit of this file. if not supplied, a default message will be used
    message: Option<String>,
    /// new_branch (optional) will make a new branch from branch before creating the file
    new_branch: Option<String>,
    /// sha is the SHA for the file that already exists
    #[skip]
    sha: String,
    /// Add a Signed-off-by trailer by the committer at the end of the commit log message.
    signoff: Option<bool>,
}

impl UpdateFileRepoBuilder {
    pub fn new(
        owner: impl ToString,
        repo: impl ToString,
        filepath: impl ToString,
        content: impl ToString,
        sha: impl ToString,
    ) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            filepath: filepath.to_string(),
            author: None,
            branch: None,
            committer: None,
            content: content.to_string(),
            dates: None,
            from_path: None,
            message: None,
            new_branch: None,
            sha: sha.to_string(),
            signoff: None,
        }
    }

    /// Send the request to update file in given repository.
    pub async fn send(&self, client: &crate::Client) -> Result<EntryMutation> {
        let owner = &self.owner;
        let repo = &self.repo;
        let filepath = &self.filepath;

        let req = client
            .put(format!("repos/{owner}/{repo}/contents/{filepath}"))
            .json(self)
            .build()?;

        let res = client.make_request(req).await?;

        client.parse_response(res).await
    }
}
