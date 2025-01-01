pub mod get;

pub struct Reviews {
    pub(crate) owner: String,
    pub(crate) repo: String,
}

impl Reviews {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    /// List a review's [Pull Request](crate::model::reviews::PullReview).
    ///
    /// # Example
    ///
    /// ```
    /// use gitea_sdk::{Client, Auth, model::issues::State};
    /// async fn reviews() {
    ///     let client = Client::new(
    ///         "https://gitea.example.com",
    ///         Auth::Token("your-token")
    ///     );
    ///     let reviews = client
    ///         .pulls("owner", "repo")
    ///         .reviews()
    ///         .get(1)
    ///         .send(&client)
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    /// This will allow you to get all the reviews from the pull request.
    pub fn get(&self, index: i64) -> get::GetReviewsBuilder {
        get::GetReviewsBuilder::new(&self.owner, &self.repo, index)
    }
}
