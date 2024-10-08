pub mod create_repo;
pub mod current;
pub mod list_repos;
pub mod orgs;
pub mod settings;
pub mod starred;
pub mod tokens;

pub struct User;

impl User {
    /// Gets the currently authenticated user.
    /// This will return a [User] object representing the currently authenticated user.
    /// As long as the token is valid, this method will always return a user.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_authenticated_user() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let user = client
    ///     .user()
    ///     .current()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    pub fn current(&self) -> current::GetAuthenticatedUserBuilder {
        current::GetAuthenticatedUserBuilder::new()
    }

    /// Creates a new repository for the authenticated user.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn create_repo() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let repo = client
    ///     .user()
    ///     .create_repo("my-new-repo")
    ///     .private(true)
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will create a new private repository with the name "my-new-repo" for the authenticated user.
    pub fn create_repo(&self, name: impl ToString) -> create_repo::CreateRepoBuilder {
        create_repo::CreateRepoBuilder::new(name)
    }

    /// Lists all repositories for the authenticated user.
    /// This will return a list of all [Repository](crate::model::repos::Repository) objects
    /// owned by the authenticated user.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn list_repos() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let repos = client
    ///     .user()
    ///     .list_repos()
    ///     .limit(10)
    ///     .page(2)
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn list_repos(&self) -> list_repos::ListReposBuilder {
        list_repos::ListReposBuilder::new()
    }

    /// List the current user's organizations.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn list_orgs() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let orgs = client
    ///     .user()
    ///     .orgs()
    ///     .page(2)
    ///     .limit(10)
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn orgs(&self) -> orgs::Orgs {
        orgs::Orgs::new()
    }

    /// Creates a new access token for a user.
    /// NOTE: This endpoint requires basic authentication and will fail otherwise.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, CreateAccessTokenOption, Auth};
    /// # async fn create_token() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Basic("username", "password")
    /// );
    /// let token = client
    ///     .user()
    ///     .create_access_token("username", "my-new-token", vec!["write:repository", "read:user"])
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// println!("Token {} created: {}", token.name, token.sha1);
    /// let new_client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token(token.sha1)
    /// );
    /// # }
    /// ```
    /// This will create a new token with the name "my-new-token", which can read all user data and
    /// read and write to repositories.
    ///
    /// If the token is successfully created, this method will return a [AccessToken] object.
    /// If the user is not authenticated correctly (e.g. not using basic auth), this method will
    /// return a 403 status code.
    /// In case of any client-side errors, this method will return a 400 status code.
    pub fn create_access_token(
        &self,
        user: impl ToString,
        name: impl ToString,
        scopes: Vec<impl ToString>,
    ) -> tokens::CreateAccessTokenBuilder {
        tokens::CreateAccessTokenBuilder::new(user, name, scopes)
    }

    /// Lists all access tokens for a user.
    /// NOTE: This endpoint requires basic authentication and will fail otherwise.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn list_tokens() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Basic("username", "password")
    /// );
    /// let tokens = client
    ///     .user()
    ///     .list_access_tokens("username")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will list all access tokens for the user "username".
    pub fn list_access_tokens(&self, username: impl ToString) -> tokens::ListAccessTokensBuilder {
        tokens::ListAccessTokensBuilder::new(username)
    }

    /// Deletes an access token by its username and token.
    /// This will delete the token and revoke all permissions associated with it.
    /// NOTE: This endpoint requires basic authentication and will fail otherwise.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn delete_token() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Basic("username", "password")
    /// );
    /// client.
    ///     user()
    ///     .delete_access_token("username", "token")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will delete the token with the name "token-name" for the user "username".
    ///
    /// If the token does not exist, this method will return a 404 status code.
    /// If the target user is not the authenticated user and the authenticated user is not an
    /// administrator, this method will return a 403 status code.
    /// For any client-side other errors, this method will return a 422 status code.
    /// If the token is successfully deleted, this method will return a 204 status code.
    pub fn delete_access_token(
        &self,
        user: impl ToString,
        token: impl ToString,
    ) -> tokens::DeleteAccessTokenBuilder {
        tokens::DeleteAccessTokenBuilder::new(user, token)
    }

    /// Gets the current user's settings.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_settings() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let settings = client
    ///     .user()
    ///     .get_settings()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// println!("User settings: {:?}", settings);
    /// # }
    /// ```
    pub fn get_settings(&self) -> settings::GetSettingsBuilder {
        settings::GetSettingsBuilder::new()
    }

    /// Updates the current user's settings.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn update_settings() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let settings = client
    ///     .user()
    ///     .update_settings()
    ///     .theme("dark")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// println!("User settings: {:?}", settings);
    /// # }
    /// ```
    /// This will update the user's theme to "dark".
    pub fn update_settings(&self) -> settings::UpdateSettingsBuilder {
        settings::UpdateSettingsBuilder::new()
    }

    /// Lists all repositories starred by the authenticated user.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn list_starred_repos() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let repos = client
    ///     .user()
    ///     .list_starred()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn list_starred(&self) -> starred::ListStarredBuilder {
        starred::ListStarredBuilder::new()
    }

    /// Checks if the authenticated user has starred a repository.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn is_starred() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let starred = client
    ///     .user()
    ///     .is_starred("owner", "repo")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// if starred {
    ///     println!("You have starred this repo!");
    /// } else {
    ///     println!("You have not starred this repo.");
    /// }
    /// # }
    /// ```
    pub fn is_starred(
        &self,
        owner: impl ToString,
        repo: impl ToString,
    ) -> starred::IsStarredBuilder {
        starred::IsStarredBuilder::new(owner, repo)
    }

    /// Stars a repository for the authenticated user.
    /// This will star the repository with the given owner and name.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn star_repo() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .user()
    ///     .star_repo("owner", "repo")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will star the repository "repo" owned by "owner".
    pub fn star_repo(&self, owner: impl ToString, repo: impl ToString) -> starred::StarRepoBuilder {
        starred::StarRepoBuilder::new(owner, repo)
    }

    /// Unstars a repository for the authenticated user.
    /// This will unstar the repository with the given owner and name.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn unstar_repo() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .user()
    ///     .unstar_repo("owner", "repo")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn unstar_repo(
        &self,
        owner: impl ToString,
        repo: impl ToString,
    ) -> starred::UnstarRepoBuilder {
        starred::UnstarRepoBuilder::new(owner, repo)
    }
}
