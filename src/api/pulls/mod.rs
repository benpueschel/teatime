pub mod create;
pub mod edit;
pub mod get;
pub mod list;
pub mod pinned;
pub mod reviews;

pub struct Pulls {
    pub(crate) owner: String,
    pub(crate) repo: String,
}

impl Pulls {
    /// Create a [Pull Request](crate::model::pulls::PullRequest) in a repository.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn create_pr() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let pr = client
    ///     .pulls("owner", "repo")
    ///     .create("my-branch", "main", "My PR")
    ///     .body("This is my PR")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will create a pull request with the title "My PR" and body "This is my PR" from the
    /// branch "my-branch" to the branch "main" in the repository "owner/repo".
    pub fn create(
        &self,
        head: impl ToString,
        base: impl ToString,
        title: impl ToString,
    ) -> create::CreatePullRequestBuilder {
        create::CreatePullRequestBuilder::new(&self.owner, &self.repo, head, base, title)
    }

    /// Edit a [Pull Request](crate::model::pulls::PullRequest) in a repository.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn edit_pr() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .pulls("owner", "repo")
    ///     .edit(1)
    ///     .title("My PR")
    ///     .body("This is my PR")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will edit the pull request with the ID 1 in the repository "owner/repo" to have the
    /// title "My PR" and body "This is my PR".
    pub fn edit(&self, id: i64) -> edit::EditPullRequestBuilder {
        edit::EditPullRequestBuilder::new(&self.owner, &self.repo, id)
    }

    /// Get a [Pull Request](crate::model::pulls::PullRequest) by its head and base branches.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_pr_by_branches() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let pr = client
    ///     .pulls("owner", "repo")
    ///     .get_by_branches("my-branch", "main")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will get the pull request from the branch "my-branch" to the branch "main" in the
    /// repository "owner/repo".
    pub fn get_by_branches(
        &self,
        head: impl ToString,
        base: impl ToString,
    ) -> get::GetPullRequestByBranchesBuilder {
        get::GetPullRequestByBranchesBuilder::new(&self.owner, &self.repo, head, base)
    }

    /// Get a [Pull Request](crate::model::pulls::PullRequest) by its ID.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_pr_by_id() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let pr = client
    ///     .pulls("owner", "repo")
    ///     .get(1)
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will get the pull request with the ID 1 in the repository "owner/repo".
    pub fn get(&self, id: i64) -> get::GetPullRequestByIdBuilder {
        get::GetPullRequestByIdBuilder::new(&self.owner, &self.repo, id)
    }

    /// List a repository's [Pull Requests](crate::model::pulls::PullRequest).
    ///
    /// # Example
    ///
    /// ```
    /// # use gitea_sdk::{Client, Auth, model::issues::State};
    /// # async fn list_prs() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let issues = client
    ///   .pulls("owner", "repo")
    ///   .list()
    ///   .state(State::Open)
    ///    .send(&client)
    ///   .await
    ///   .unwrap();
    /// # }
    /// ```
    /// This will get all open issues in the repository "owner/repo".
    pub fn list(&self) -> list::ListPullRequestsBuilder {
        list::ListPullRequestsBuilder::new(&self.owner, &self.repo)
    }

    /// Get a list of pinned [Pull Requests](crate::model::pulls::PullRequest) in a repository.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn pinned_prs() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let pinned_prs = client
    ///     .pulls("owner", "repo")
    ///     .pinned()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will get all pinned pull requests in the repository "owner/repo".
    pub fn pinned(&self) -> pinned::PinnedPullRequestsBuilder {
        pinned::PinnedPullRequestsBuilder::new(&self.owner, &self.repo)
    }

    pub fn reviews(&self) -> reviews::Reviews {
        reviews::Reviews::new(&self.owner, &self.repo)
    }
}
