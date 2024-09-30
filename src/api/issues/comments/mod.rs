pub mod create;
pub mod delete;
pub mod edit;
pub mod get;
pub mod list;

pub struct Comments {
    pub(crate) owner: String,
    pub(crate) repo: String,
}

impl Comments {
    /// Create a comment on an issue.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn create_comment() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let comment = client
    ///     .issues("owner", "repo")
    ///     .comments()
    ///     .create(1, "This is a comment")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn create(&self, issue: i64, body: impl ToString) -> create::CreateCommentBuilder {
        create::CreateCommentBuilder::new(&self.owner, &self.repo, issue, body)
    }
    /// List an issue's comments.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn list_comments() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let comments = client
    ///     .issues("owner", "repo")
    ///     .comments()
    ///     .list(1)
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn list(&self, issue: i64) -> list::ListCommentsBuilder {
        list::ListCommentsBuilder::new(&self.owner, &self.repo, issue)
    }
    /// List all comments in a repository.
    /// This will return a list of all comments in the repository.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn list_all_comments() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let comments = client
    ///     .issues("owner", "repo")
    ///     .comments()
    ///     .list_all()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn list_all(&self) -> list::ListAllCommentsBuilder {
        list::ListAllCommentsBuilder::new(&self.owner, &self.repo)
    }

    /// Edit a comment on an issue.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn edit_comment() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let comment = client
    ///     .issues("owner", "repo")
    ///     .comments()
    ///     .edit(1, "This is an edited comment")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn edit(&self, comment: i64, body: impl ToString) -> edit::EditCommentBuilder {
        edit::EditCommentBuilder::new(&self.owner, &self.repo, comment, body)
    }

    /// Delete a comment on an issue.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn delete_comment() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .issues("owner", "repo")
    ///     .comments()
    ///     .delete(1)
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn delete(&self, comment: i64) -> delete::DeleteCommentBuilder {
        delete::DeleteCommentBuilder::new(&self.owner, &self.repo, comment)
    }
}
