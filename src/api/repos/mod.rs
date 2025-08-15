pub mod branches;
pub mod commits;
pub mod delete;
pub mod edit;
pub mod forks;
pub mod generate;
pub mod get;

/// The [Repos] struct provides methods for interacting with repositories.
pub struct Repos {
    pub(crate) owner: String,
    pub(crate) repo: String,
}

impl Repos {
    /// Create a repository using a template
    ///
    /// # Example
    /// ```rust
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn generate_repo() {
    /// let client = Client::new(
    ///    "https://gitea.example.com",
    ///    Auth::Token("your-token")
    /// );
    /// client
    ///    .repos("owner", "repo") // New repository to create
    ///    .generate("template-owner", "template-repo") // Repository to use as template
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    pub fn generate(
        &self,
        template_owner: impl ToString,
        template_repo: impl ToString,
    ) -> generate::GenerateRepoBuilder {
        generate::GenerateRepoBuilder::new(&self.owner, &self.repo, template_owner, template_repo)
    }
    /// Deletes a repository by its owner and name.
    /// WARNING: This is irreversible and will delete all data associated with the repository.
    /// This action cannot be undone. When invoking this method, you will not be asked for
    /// confirmation. Use with caution, please don't nuke your repositories.
    ///
    /// # Example
    /// ```rust
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn delete_repo() {
    /// let client = Client::new(
    ///    "https://gitea.example.com",
    ///    Auth::Token("your-token")
    /// );
    /// client
    ///    .repos("owner", "repo")
    ///    .delete()
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    pub fn delete(&self) -> delete::DeleteRepoBuilder {
        delete::DeleteRepoBuilder::new(&self.owner, &self.repo)
    }
    /// Gets a repository by its owner and name.
    /// This will return a [Repository] object if the repository exists and is visible to the
    /// currently authenticated user.
    ///
    /// # Example
    /// ```rust
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_repo() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let repo = client
    ///     .repos("owner", "repo")
    ///     .get()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    ///
    pub fn get(&self) -> get::GetRepoBuilder {
        get::GetRepoBuilder::new(&self.owner, &self.repo)
    }

    /// Forks a repository by its owner and name.
    /// The [CreateForkOption] struct provides a number of optional fields to customize the fork,
    /// but all fields are entirely optional.
    /// If you don't set any fields, the fork will be created with the same name as the original
    /// repository in the authenticated user's account.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn fork_repo() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let forked_repo = client
    ///     .repos("owner", "repo")
    ///     .create_fork()
    ///     .name("my-fork")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will fork the repository "owner/repo" into the authenticated user's account with the
    /// name "my-fork".
    ///
    /// ```rust
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn fork_repo() {
    /// let client = Client::new(
    ///    "https://gitea.example.com",
    ///    Auth::Token("your-token")
    /// );
    /// let forked_repo = client
    ///    .repos("owner", "repo")
    ///    .create_fork()
    ///    .organization("my-org")
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    /// This will fork the repository "owner/repo" into the organization "my-org" with the same
    /// name as the original repository.
    ///
    /// ```rust
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn fork_repo() {
    /// let client = Client::new(
    ///    "https://gitea.example.com",
    ///    Auth::Token("your-token")
    /// );
    /// let created_repo = client
    ///   .user()
    ///   .create_repo("my-new-repo")
    ///   .send(&client)
    ///   .await
    ///   .unwrap();
    /// let forked_repo = client
    ///    .repos("owner", "my-new-repo")
    ///    .create_fork()
    ///    .name("my-new-repo")
    ///    .send(&client)
    ///    .await
    ///    .expect_err("Repository with the same name should already exist for the current user");
    /// # }
    /// ```
    /// This will create a new repository with the name "my-new-repo" for the authenticated user,
    /// then attempt to fork the repository "owner/repo" into the authenticated user's account.
    /// The fork will fail because a repository with the same name already exists.
    pub fn create_fork(&self) -> forks::CreateForkBuilder {
        forks::CreateForkBuilder::new(&self.owner, &self.repo)
    }

    /// Edits a repository by its owner and name.
    ///
    /// If you don't set any fields, the repository will not be modified.
    ///
    /// # Example
    /// ```rust
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn edit_repo() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///      Auth::Token("your-token")
    /// );
    /// let repo = client
    ///     .repos("owner", "repo")
    ///     .edit()
    ///     .description("A new description")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will set the description of the repository "owner/repo" to "A new description".
    /// If you want to remove the description, you can set it to an empty string.
    pub fn edit(&self) -> edit::EditRepoBuilder {
        edit::EditRepoBuilder::new(&self.owner, &self.repo)
    }

    /// Lists the forks of a repository by its owner and name.
    pub fn get_forks(&self) -> forks::ListForksBuilder {
        forks::ListForksBuilder::new(&self.owner, &self.repo)
    }

    /// Gets a list of commits for a repository.
    /// The [GetCommitsOption] struct provides a number of optional fields to filter the results,
    /// but all fields are entirely optional.
    /// If you don't set any fields, you will get the most recent commits for the default branch.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, api::repos::commits::GetCommitsBuilder, Auth};
    /// # async fn get_commits() {
    /// let client = Client::new(
    ///    "https://gitea.example.com",
    ///    Auth::Token("your-token")
    /// );
    /// let commits = client
    ///   .repos("owner", "repo")
    ///   .get_commits()
    ///   .send(&client)
    ///   .await
    ///   .unwrap();
    /// # }
    /// ```
    pub fn get_commits(&self) -> commits::GetCommitsBuilder {
        commits::GetCommitsBuilder::new(&self.owner, &self.repo)
    }

    /// Lists a repository's branches.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn list_branches() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let branches = client
    ///     .repos("owner", "repo")
    ///     .list_branches()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will list the branches for the repository "owner/repo".
    pub fn list_branches(&self) -> branches::ListBranchesBuilder {
        branches::ListBranchesBuilder::new(&self.owner, &self.repo)
    }

    /// Creates a new branch in a repository.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn create_branch() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .repos("owner", "repo")
    ///     .create_branch("new-branch")
    ///     .old_ref_name("main")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will create a new branch named "new-branch" in the repository "owner/repo" based on
    /// the "main" branch.
    ///
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn create_branch() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .repos("owner", "repo")
    ///     .create_branch("new-branch")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will create a new empty branch named "new-branch" in the repository "owner/repo"
    pub fn create_branch(&self, new_branch_name: impl ToString) -> branches::CreateBranchBuilder {
        branches::CreateBranchBuilder::new(&self.owner, &self.repo, new_branch_name)
    }

    /// Gets a branch in a repository.
    /// This will return a [Branch](crate::model::repos::Branch) object if the branch exists.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_branch() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let branch = client
    ///     .repos("owner", "repo")
    ///     .get_branch("main")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will get the branch "main" in the repository "owner/repo".
    pub fn get_branch(&self, branch: impl ToString) -> branches::GetBranchBuilder {
        branches::GetBranchBuilder::new(&self.owner, &self.repo, branch)
    }

    /// Deletes a branch in a repository.
    /// WARNING: This is irreversible and will delete all data associated with the branch.
    /// This action cannot be undone. When invoking this method, you will not be asked for
    /// confirmation. Use with caution
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn delete_branch() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .repos("owner", "repo")
    ///     .delete_branch("branch-to-delete")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will delete the branch "branch-to-delete" in the repository "owner/repo".
    pub fn delete_branch(&self, branch: impl ToString) -> branches::DeleteBranchBuilder {
        branches::DeleteBranchBuilder::new(&self.owner, &self.repo, branch)
    }
}
