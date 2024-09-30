pub mod comments;
pub mod create;
pub mod delete;
pub mod edit;
pub mod get;
pub mod list;

pub struct Issues {
    pub(crate) owner: String,
    pub(crate) repo: String,
}

impl Issues {
    /// Create an issue.
    /// If using deadline only the date will be taken into account, and time of day ignored.
    /// The only required field in the [CreateIssueOption] is `title`. All other fields are
    /// optional.
    /// This method will return the created issue.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn create_issue() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let issue = client
    ///    .issues("owner", "repo")
    ///    .create("my-new-issue")
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    /// This will create a new issue with the title "my-new-issue" in the repository "owner/repo".
    pub fn create(&self, title: impl ToString) -> create::CreateIssueBuilder {
        create::CreateIssueBuilder::new(&self.owner, &self.repo, title)
    }
    /// Delete an issue.
    /// This will delete the issue with the given issue number.
    /// WARNING: This is irreversible and will not ask for confirmation. Use with caution.
    ///
    /// This method will return a 204 status code if the issue was successfully deleted.
    /// If the issue does not exist, this method will return a 404 status code.
    /// If the user is not authorized to delete the issue, this method will return a 403 status
    /// code.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn delete_issue() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///    .issues("owner", "repo")
    ///    .delete(1)
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    /// This will delete the issue #1 in the repository "owner/repo".
    pub fn delete(&self, issue_number: i64) -> delete::DeleteIssueBuilder {
        delete::DeleteIssueBuilder::new(&self.owner, &self.repo, issue_number)
    }

    /// Get an issue.
    /// This will return the issue with the given issue number.
    ///
    /// # Example
    /// ```rust
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_issue() {
    /// let client = Client::new(
    ///    "https://gitea.example.com",
    ///    Auth::Token("your-token")
    /// );
    /// let issue = client
    ///     .issues("owner", "repo")
    ///     .get(1)
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will get the issue #1 in the repository "owner/repo".
    pub fn get(&self, issue_number: i64) -> get::GetIssueBuilder {
        get::GetIssueBuilder::new(&self.owner, &self.repo, issue_number)
    }

    /// Edit an issue.
    ///
    /// # Example
    /// ```rust
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn edit_issue() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .issues("owner", "repo")
    ///     .edit(1)
    ///     .title("new-title")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn edit(&self, issue_number: i64) -> edit::EditIssueBuilder {
        edit::EditIssueBuilder::new(&self.owner, &self.repo, issue_number)
    }

    /// List a repository's issues.
    /// The [GetIssuesOption] struct provides a number of optional fields to filter the results,
    /// but all fields are entirely optional.
    /// If you don't set any fields, you will get the most recent issues for the repository.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// # use gitea_sdk::{Client, Auth, model::issues::State};
    /// # async fn get_issues() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let issues = client
    ///   .issues("owner", "repo")
    ///   .list()
    ///   .state(State::Open)
    ///    .send(&client)
    ///   .await
    ///   .unwrap();
    /// # }
    /// ```
    /// This will get all open issues in the repository "owner/repo".
    pub fn list(&self) -> list::ListIssuesBuilder {
        list::ListIssuesBuilder::new(&self.owner, &self.repo)
    }

    /// Miscellaneous methods for comments on issues.
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
    ///     .comments();
    /// }
    /// ```
    pub fn comments(&self) -> comments::Comments {
        comments::Comments {
            owner: self.owner.clone(),
            repo: self.repo.clone(),
        }
    }
}
