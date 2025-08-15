use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::repos::Repository};

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[build_it(into)]
#[serde(default)]
pub struct GenerateRepoBuilder {
    /// The template repository owner
    #[skip]
    #[serde(skip)]
    template_owner: String,
    /// The template repository name
    #[skip]
    #[serde(skip)]
    template_repo: String,
    /// Include avatar of the template repo
    avatar: Option<bool>,
    /// Default branch of the new repository
    default_branch: Option<bool>,
    /// Description of the repository to create
    description: Option<String>,
    /// Include git content of default branch in template repo, set as true by default
    git_content: Option<bool>,
    /// Include git hooks in template repo
    git_hooks: Option<bool>,
    /// Include labels in template repo
    labels: Option<bool>,
    /// The name of the repository to create, must be unique.
    #[skip]
    name: String,
    /// The owner of the repository to create.
    #[skip]
    owner: String,
    /// Whether the repository is private
    private: Option<bool>,
    /// Include topics in template repo
    topics: Option<bool>,
    /// Include webhooks in template repo
    webhooks: Option<bool>,
}

impl GenerateRepoBuilder {
    pub fn new(
        owner: impl ToString,
        repo: impl ToString,
        template_owner: impl ToString,
        template_repo: impl ToString,
    ) -> Self {
        Self {
            template_owner: template_owner.to_string(),
            template_repo: template_repo.to_string(),
            avatar: None,
            default_branch: None,
            description: None,
            // Set as true by default else we get vague error, most users will likely mean this when
            // using template
            git_content: Some(true),
            git_hooks: None,
            labels: None,
            name: repo.to_string(),
            owner: owner.to_string(),
            private: None,
            topics: None,
            webhooks: None,
        }
    }
    /// Send the request to generate the repository.
    pub async fn send(&self, client: &crate::Client) -> Result<Repository> {
        let template_owner = &self.template_owner;
        let template_repo = &self.template_repo;
        let req = client
            .post(format!("repos/{template_owner}/{template_repo}/generate"))
            .json(&self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
