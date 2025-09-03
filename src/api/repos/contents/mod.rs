use crate::api::repos::contents;

pub mod get;

pub struct Contents {
    pub(crate) owner: String,
    pub(crate) repo: String,
}

impl Contents {
    /// Gets the metadata and contents (if a file) of an entry in a repository, or a list of entries if a dir
    /// This will return a list of all [Entry](crate::model::repos::Entry) objects
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn fetch_repo_content() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let src_entries = client
    ///     .repos("repo-owner", "repo-name")
    ///     .contents()
    ///     .get("src")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    ///
    /// let main_file_entry = client
    ///     .repos("repo-owner", "repo-name")
    ///     .contents()
    ///     .get("src/main.rs")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn get(&self, filepath: impl ToString) -> contents::get::GetContentsRepoBuilder {
        contents::get::GetContentsRepoBuilder::new(self.owner.clone(), self.repo.clone(), filepath)
    }
}
