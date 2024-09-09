use build_it::Builder;
use serde::Serialize;

use crate::{
    error::Result,
    model::repos::{ExternalTracker, ExternalWiki, Repository},
};

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[build_it(into)]
#[serde(default)]
pub struct EditRepoBuilder {
    #[skip]
    #[serde(skip)]
    owner: String,
    #[skip]
    #[serde(skip)]
    repo: String,

    /// Either `true` to allow fast-forward-only merging pull requests, or `false` to prevent fast-forward-only merging.
    allow_fast_forward_only_merge: Option<bool>,
    /// Either `true` to allow mark pr as merged manually, or `false` to prevent it.
    allow_manual_merge: Option<bool>,
    /// Either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits.
    allow_merge_commits: Option<bool>,
    /// Either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging.
    allow_rebase: Option<bool>,
    /// Either `true` to allow rebase with explicit merge commits (--no-ff), or `false` to prevent rebase with explicit merge commits.
    allow_rebase_explicit: Option<bool>,
    /// Either `true` to allow updating pull request branch by rebase, or `false` to prevent it.
    allow_rebase_update: Option<bool>,
    /// Either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging.
    allow_squash_merge: Option<bool>,
    /// Set to `true` to archive this repository.
    archived: Option<bool>,
    /// Either `true` to enable AutodetectManualMerge, or `false` to prevent it. Note: In some special cases, misjudgments can occur.
    autodetect_manual_merge: Option<bool>,
    /// Set to `true` to allow edits from maintainers by default
    default_allow_maintainer_edit: Option<bool>,
    /// Sets the default branch for this repository.
    default_branch: Option<String>,
    /// Set to `true` to delete pr branch after merge by default
    default_delete_branch_after_merge: Option<bool>,
    /// Set to a merge style to be used by this repository: "merge", "rebase", "rebase-merge", "squash", or "fast-forward-only".
    default_merge_style: Option<String>,
    /// A short description of the repository.
    description: Option<String>,
    /// Enable prune - remove obsolete remote-tracking references when mirroring
    enable_prune: Option<bool>,
    /// ExternalTracker represents settings for external tracker
    external_tracker: Option<ExternalTracker>,
    /// ExternalWiki represents setting for external wiki
    external_wiki: Option<ExternalWiki>,
    /// Either `true` to enable actions unit, or `false` to disable them.
    has_actions: Option<bool>,
    /// Either `true` to enable issues for this repository or `false` to disable them.
    has_issues: Option<bool>,
    /// Either `true` to enable packages unit, or `false` to disable them.
    has_packages: Option<bool>,
    /// Either `true` to enable project unit, or `false` to disable them.
    has_projects: Option<bool>,
    /// Either `true` to allow pull requests, or `false` to prevent pull request.
    has_pull_requests: Option<bool>,
    /// Either `true` to enable releases unit, or `false` to disable them.
    has_releases: Option<bool>,
    /// Either `true` to enable the wiki for this repository or `false` to disable it.
    has_wiki: Option<bool>,
    /// Either `true` to ignore whitespace for conflicts, or `false` to not ignore whitespace.
    ignore_whitespace_conflicts: Option<bool>,
    /// Set to a string like `8h30m0s` to set the mirror interval time
    mirror_interval: Option<String>,
    /// Name of the repository
    name: Option<String>,
    /// Either `true` to make the repository private or `false` to make it public.
    ///
    /// NOTE: you will get a 422 error if the organization restricts changing repository visibility
    /// To organization owners and a non-owner tries to change the value of private.
    private: Option<bool>,
    /// `repo` to only allow repo-level projects, `owner` to only allow owner projects, `all` to allow both.
    projects_mode: Option<String>,
    /// Either `true` to make this repository a template or `false` to make it a normal repository
    template: Option<bool>,
    /// A URL with more information about the repository.
    website: Option<String>,
}

impl EditRepoBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            allow_fast_forward_only_merge: None,
            allow_manual_merge: None,
            allow_merge_commits: None,
            allow_rebase: None,
            allow_rebase_explicit: None,
            allow_rebase_update: None,
            allow_squash_merge: None,
            archived: None,
            autodetect_manual_merge: None,
            default_allow_maintainer_edit: None,
            default_branch: None,
            default_delete_branch_after_merge: None,
            default_merge_style: None,
            description: None,
            enable_prune: None,
            external_tracker: None,
            external_wiki: None,
            has_actions: None,
            has_issues: None,
            has_packages: None,
            has_projects: None,
            has_pull_requests: None,
            has_releases: None,
            has_wiki: None,
            ignore_whitespace_conflicts: None,
            mirror_interval: None,
            name: None,
            private: None,
            projects_mode: None,
            template: None,
            website: None,
        }
    }
    /// Send the request to edit the repository.
    pub async fn send(&self, client: &crate::Client) -> Result<Repository> {
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .patch(format!("repos/{owner}/{repo}"))
            .json(&self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
