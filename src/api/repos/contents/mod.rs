use crate::api::repos::contents;

pub mod create_file;
pub mod delete_file;
pub mod get;
pub mod update_file;

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

    /// Create a file in a repository
    /// This will return [EntryMutation](crate::model::repos::EntryMutation) object
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn create_new_file() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let new_entry = client
    ///    .repos("repo-owner", "repo-name")
    ///    .contents()
    ///    .create_file("test/server.yml", BASE64_STANDARD.encode(b"port: 80"))
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    pub fn create_file(
        &self,
        filepath: impl ToString,
        content: impl ToString,
    ) -> contents::create_file::CreateFileRepoBuilder {
        contents::create_file::CreateFileRepoBuilder::new(
            self.owner.clone(),
            self.repo.clone(),
            filepath,
            content,
        )
    }

    /// Update a file in a repository
    /// This will return [EntryMutation](crate::model::repos::EntryMutation) object
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn update_file() {
    ///
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    ///
    /// let entries = client
    ///     .repos("repo-owner", "repo-name")
    ///     .contents()
    ///     .get("test/server.yml")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    ///
    /// let server_entry = &entries[0];
    ///
    /// let entry_mutation = client
    ///     .repos("repo-owner", "repo-name")
    ///     .contents()
    ///     .update_file(
    ///         "test/server.yml",
    ///         BASE64_STANDARD.encode(b"port: 8080"),
    ///         server_entry.sha.clone(),
    ///     )
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn update_file(
        &self,
        filepath: impl ToString,
        content: impl ToString,
        sha: impl ToString,
    ) -> contents::update_file::UpdateFileRepoBuilder {
        contents::update_file::UpdateFileRepoBuilder::new(
            self.owner.clone(),
            self.repo.clone(),
            filepath,
            content,
            sha,
        )
    }

    /// Delete a file in a repository
    /// This will return [EntryMutation](crate::model::repos::EntryMutation) object
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn delete_file() {
    ///
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    ///
    /// let entries = client
    ///     .repos("repo-owner", "repo-name")
    ///     .contents()
    ///     .get("test/server.yml")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    ///
    /// let server_entry = &entries[0];
    ///
    /// let entry_mutation = client
    ///     .repos("repo-owner", "repo-name")
    ///     .contents()
    ///     .delete_file("test/server.yml", server_entry.sha.clone())
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn delete_file(
        &self,
        filepath: impl ToString,
        sha: impl ToString,
    ) -> contents::delete_file::DeleteFileRepoBuilder {
        contents::delete_file::DeleteFileRepoBuilder::new(
            self.owner.clone(),
            self.repo.clone(),
            filepath,
            sha,
        )
    }
}
