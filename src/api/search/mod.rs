pub mod issues;
pub mod repos;
pub mod users;

pub struct Search;

impl Search {
    /// Searches for repositories based on the given search options.
    /// All fields in the [SearchRepositoriesOption] are optional.
    /// This method will return a list of repositories that match the search criteria.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn search_repos() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let repo = client
    ///     .search()
    ///     .repos()
    ///     .query("my-repo")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will search for repositories matching the keyword "my-repo".
    /// The search will include the repository description and will return the first page of
    /// result.
    pub fn repos(&self) -> repos::SearchRepositoriesBuilder {
        repos::SearchRepositoriesBuilder::new()
    }

    /// Searches for users based on the given search options.
    /// This method will return a list of users that match the search criteria.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn search_users() {
    /// let client = Client::new(
    ///    "https://gitea.example.com",
    ///    Auth::Token("your-token")
    /// );
    /// let users = client
    ///    .search()
    ///    .users()
    ///    .query("my-user")
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    pub fn users(&self) -> users::SearchUsersBuilder {
        users::SearchUsersBuilder::new()
    }

    /// Searches for issues based on the given search options.
    /// This method will return a list of issues that match the search criteria.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn search_issues() {
    /// let client = Client::new(
    ///   "https://gitea.example.com",
    ///   Auth::Token("your-token")
    /// );
    /// let issues = client
    ///     .search()
    ///     .issues()
    ///     .query("my-issue")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will search for issues matching the keyword "my-issue".
    pub fn issues(&self) -> issues::SearchIssuesBuilder {
        issues::SearchIssuesBuilder::new()
    }
}
