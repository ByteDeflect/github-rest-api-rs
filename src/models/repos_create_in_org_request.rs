/*
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * The version of the OpenAPI document: 1.1.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReposCreateInOrgRequest {
    /// The name of the repository.
    #[serde(rename = "name")]
    pub name: String,
    /// A short description of the repository.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A URL with more information about the repository.
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// Whether the repository is private.
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /// The visibility of the repository.
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    /// Either `true` to enable issues for this repository or `false` to disable them.
    #[serde(rename = "has_issues", skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    /// Either `true` to enable projects for this repository or `false` to disable them. **Note:** If you're creating a repository in an organization that has disabled repository projects, the default is `false`, and if you pass `true`, the API returns an error.
    #[serde(rename = "has_projects", skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    /// Either `true` to enable the wiki for this repository or `false` to disable it.
    #[serde(rename = "has_wiki", skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    /// Whether downloads are enabled.
    #[serde(rename = "has_downloads", skip_serializing_if = "Option::is_none")]
    pub has_downloads: Option<bool>,
    /// Either `true` to make this repo available as a template repository or `false` to prevent it.
    #[serde(rename = "is_template", skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    /// The id of the team that will be granted access to this repository. This is only valid when creating a repository in an organization.
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i32>,
    /// Pass `true` to create an initial commit with empty README.
    #[serde(rename = "auto_init", skip_serializing_if = "Option::is_none")]
    pub auto_init: Option<bool>,
    /// Desired language or platform [.gitignore template](https://github.com/github/gitignore) to apply. Use the name of the template without the extension. For example, \"Haskell\".
    #[serde(rename = "gitignore_template", skip_serializing_if = "Option::is_none")]
    pub gitignore_template: Option<String>,
    /// Choose an [open source license template](https://choosealicense.com/) that best suits your needs, and then use the [license keyword](https://docs.github.com/articles/licensing-a-repository/#searching-github-by-license-type) as the `license_template` string. For example, \"mit\" or \"mpl-2.0\".
    #[serde(rename = "license_template", skip_serializing_if = "Option::is_none")]
    pub license_template: Option<String>,
    /// Either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging.
    #[serde(rename = "allow_squash_merge", skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    /// Either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits.
    #[serde(rename = "allow_merge_commit", skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /// Either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging.
    #[serde(rename = "allow_rebase_merge", skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    /// Either `true` to allow auto-merge on pull requests, or `false` to disallow auto-merge.
    #[serde(rename = "allow_auto_merge", skip_serializing_if = "Option::is_none")]
    pub allow_auto_merge: Option<bool>,
    /// Either `true` to allow automatically deleting head branches when pull requests are merged, or `false` to prevent automatic deletion. **The authenticated user must be an organization owner to set this property to `true`.**
    #[serde(rename = "delete_branch_on_merge", skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    /// Either `true` to allow squash-merge commits to use pull request title, or `false` to use commit message. **This property has been deprecated. Please use `squash_merge_commit_title` instead.
    #[serde(rename = "use_squash_pr_title_as_default", skip_serializing_if = "Option::is_none")]
    pub use_squash_pr_title_as_default: Option<bool>,
    /// Required when using `squash_merge_commit_message`.  The default value for a squash merge commit title:  - `PR_TITLE` - default to the pull request's title. - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).
    #[serde(rename = "squash_merge_commit_title", skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_title: Option<SquashMergeCommitTitle>,
    /// The default value for a squash merge commit message:  - `PR_BODY` - default to the pull request's body. - `COMMIT_MESSAGES` - default to the branch's commit messages. - `BLANK` - default to a blank commit message.
    #[serde(rename = "squash_merge_commit_message", skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_message: Option<SquashMergeCommitMessage>,
    /// Required when using `merge_commit_message`.  The default value for a merge commit title.  - `PR_TITLE` - default to the pull request's title. - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name).
    #[serde(rename = "merge_commit_title", skip_serializing_if = "Option::is_none")]
    pub merge_commit_title: Option<MergeCommitTitle>,
    /// The default value for a merge commit message.  - `PR_TITLE` - default to the pull request's title. - `PR_BODY` - default to the pull request's body. - `BLANK` - default to a blank commit message.
    #[serde(rename = "merge_commit_message", skip_serializing_if = "Option::is_none")]
    pub merge_commit_message: Option<MergeCommitMessage>,
    /// The custom properties for the new repository. The keys are the custom property names, and the values are the corresponding custom property values.
    #[serde(rename = "custom_properties", skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl ReposCreateInOrgRequest {
    pub fn new(name: String) -> ReposCreateInOrgRequest {
        ReposCreateInOrgRequest {
            name,
            description: None,
            homepage: None,
            private: None,
            visibility: None,
            has_issues: None,
            has_projects: None,
            has_wiki: None,
            has_downloads: None,
            is_template: None,
            team_id: None,
            auto_init: None,
            gitignore_template: None,
            license_template: None,
            allow_squash_merge: None,
            allow_merge_commit: None,
            allow_rebase_merge: None,
            allow_auto_merge: None,
            delete_branch_on_merge: None,
            use_squash_pr_title_as_default: None,
            squash_merge_commit_title: None,
            squash_merge_commit_message: None,
            merge_commit_title: None,
            merge_commit_message: None,
            custom_properties: None,
        }
    }
}
/// The visibility of the repository.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Self::Public
    }
}
/// Required when using `squash_merge_commit_message`.  The default value for a squash merge commit title:  - `PR_TITLE` - default to the pull request's title. - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SquashMergeCommitTitle {
    #[serde(rename = "PR_TITLE")]
    PrTitle,
    #[serde(rename = "COMMIT_OR_PR_TITLE")]
    CommitOrPrTitle,
}

impl Default for SquashMergeCommitTitle {
    fn default() -> SquashMergeCommitTitle {
        Self::PrTitle
    }
}
/// The default value for a squash merge commit message:  - `PR_BODY` - default to the pull request's body. - `COMMIT_MESSAGES` - default to the branch's commit messages. - `BLANK` - default to a blank commit message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SquashMergeCommitMessage {
    #[serde(rename = "PR_BODY")]
    PrBody,
    #[serde(rename = "COMMIT_MESSAGES")]
    CommitMessages,
    #[serde(rename = "BLANK")]
    Blank,
}

impl Default for SquashMergeCommitMessage {
    fn default() -> SquashMergeCommitMessage {
        Self::PrBody
    }
}
/// Required when using `merge_commit_message`.  The default value for a merge commit title.  - `PR_TITLE` - default to the pull request's title. - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MergeCommitTitle {
    #[serde(rename = "PR_TITLE")]
    PrTitle,
    #[serde(rename = "MERGE_MESSAGE")]
    MergeMessage,
}

impl Default for MergeCommitTitle {
    fn default() -> MergeCommitTitle {
        Self::PrTitle
    }
}
/// The default value for a merge commit message.  - `PR_TITLE` - default to the pull request's title. - `PR_BODY` - default to the pull request's body. - `BLANK` - default to a blank commit message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MergeCommitMessage {
    #[serde(rename = "PR_BODY")]
    PrBody,
    #[serde(rename = "PR_TITLE")]
    PrTitle,
    #[serde(rename = "BLANK")]
    Blank,
}

impl Default for MergeCommitMessage {
    fn default() -> MergeCommitMessage {
        Self::PrBody
    }
}

