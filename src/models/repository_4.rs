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

/// Repository4 : A git repository
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Repository4 {
    /// Whether to allow auto-merge for pull requests.
    #[serde(rename = "allow_auto_merge", skip_serializing_if = "Option::is_none")]
    pub allow_auto_merge: Option<bool>,
    /// Whether to allow private forks
    #[serde(rename = "allow_forking", skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    /// Whether to allow merge commits for pull requests.
    #[serde(rename = "allow_merge_commit", skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /// Whether to allow rebase merges for pull requests.
    #[serde(rename = "allow_rebase_merge", skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    /// Whether to allow squash merges for pull requests.
    #[serde(rename = "allow_squash_merge", skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(rename = "allow_update_branch", skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    /// Whether the repository is archived.
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "created_at")]
    pub created_at: Box<models::RepositoryCreatedAt>,
    /// The default branch of the repository.
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    /// Whether to delete head branches when pull requests are merged
    #[serde(rename = "delete_branch_on_merge", skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// Returns whether or not this repository is disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "fork")]
    pub fork: bool,
    #[serde(rename = "forks")]
    pub forks: i32,
    #[serde(rename = "forks_count")]
    pub forks_count: i32,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "git_url")]
    pub git_url: String,
    /// Whether downloads are enabled.
    #[serde(rename = "has_downloads")]
    pub has_downloads: bool,
    /// Whether issues are enabled.
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_pages")]
    pub has_pages: bool,
    /// Whether projects are enabled.
    #[serde(rename = "has_projects")]
    pub has_projects: bool,
    /// Whether the wiki is enabled.
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    /// Whether discussions are enabled.
    #[serde(rename = "has_discussions")]
    pub has_discussions: bool,
    #[serde(rename = "homepage", deserialize_with = "Option::deserialize")]
    pub homepage: Option<String>,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    /// Unique identifier of the repository
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "is_template", skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "language", deserialize_with = "Option::deserialize")]
    pub language: Option<String>,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "license", deserialize_with = "Option::deserialize")]
    pub license: Option<Box<models::License>>,
    #[serde(rename = "master_branch", skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    /// The default value for a merge commit message.  - `PR_TITLE` - default to the pull request's title. - `PR_BODY` - default to the pull request's body. - `BLANK` - default to a blank commit message.
    #[serde(rename = "merge_commit_message", skip_serializing_if = "Option::is_none")]
    pub merge_commit_message: Option<MergeCommitMessage>,
    /// The default value for a merge commit title.  - `PR_TITLE` - default to the pull request's title. - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name).
    #[serde(rename = "merge_commit_title", skip_serializing_if = "Option::is_none")]
    pub merge_commit_title: Option<MergeCommitTitle>,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "mirror_url", deserialize_with = "Option::deserialize")]
    pub mirror_url: Option<String>,
    /// The name of the repository.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "open_issues")]
    pub open_issues: i32,
    #[serde(rename = "open_issues_count")]
    pub open_issues_count: i32,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "owner", deserialize_with = "Option::deserialize")]
    pub owner: Option<Box<models::User>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<models::RepositoryPermissions>>,
    /// Whether the repository is private or public.
    #[serde(rename = "private")]
    pub private: bool,
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "pushed_at", deserialize_with = "Option::deserialize")]
    pub pushed_at: Option<Box<models::RepositoryPushedAt>>,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "role_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role_name: Option<Option<String>>,
    #[serde(rename = "size")]
    pub size: i32,
    /// The default value for a squash merge commit message:  - `PR_BODY` - default to the pull request's body. - `COMMIT_MESSAGES` - default to the branch's commit messages. - `BLANK` - default to a blank commit message.
    #[serde(rename = "squash_merge_commit_message", skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_message: Option<SquashMergeCommitMessage>,
    /// The default value for a squash merge commit title:  - `PR_TITLE` - default to the pull request's title. - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).
    #[serde(rename = "squash_merge_commit_title", skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_title: Option<SquashMergeCommitTitle>,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "stargazers", skip_serializing_if = "Option::is_none")]
    pub stargazers: Option<i32>,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: i32,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "svn_url")]
    pub svn_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "topics")]
    pub topics: Vec<String>,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "url")]
    pub url: String,
    /// Whether a squash merge commit can use the pull request title as default. **This property has been deprecated. Please use `squash_merge_commit_title` instead.
    #[serde(rename = "use_squash_pr_title_as_default", skip_serializing_if = "Option::is_none")]
    pub use_squash_pr_title_as_default: Option<bool>,
    #[serde(rename = "visibility")]
    pub visibility: Visibility,
    #[serde(rename = "watchers")]
    pub watchers: i32,
    #[serde(rename = "watchers_count")]
    pub watchers_count: i32,
    /// Whether to require contributors to sign off on web-based commits
    #[serde(rename = "web_commit_signoff_required", skip_serializing_if = "Option::is_none")]
    pub web_commit_signoff_required: Option<bool>,
}

impl Repository4 {
    /// A git repository
    pub fn new(archive_url: String, archived: bool, assignees_url: String, blobs_url: String, branches_url: String, clone_url: String, collaborators_url: String, comments_url: String, commits_url: String, compare_url: String, contents_url: String, contributors_url: String, created_at: models::RepositoryCreatedAt, default_branch: String, deployments_url: String, description: Option<String>, downloads_url: String, events_url: String, fork: bool, forks: i32, forks_count: i32, forks_url: String, full_name: String, git_commits_url: String, git_refs_url: String, git_tags_url: String, git_url: String, has_downloads: bool, has_issues: bool, has_pages: bool, has_projects: bool, has_wiki: bool, has_discussions: bool, homepage: Option<String>, hooks_url: String, html_url: String, id: i64, issue_comment_url: String, issue_events_url: String, issues_url: String, keys_url: String, labels_url: String, language: Option<String>, languages_url: String, license: Option<models::License>, merges_url: String, milestones_url: String, mirror_url: Option<String>, name: String, node_id: String, notifications_url: String, open_issues: i32, open_issues_count: i32, owner: Option<models::User>, private: bool, pulls_url: String, pushed_at: Option<models::RepositoryPushedAt>, releases_url: String, size: i32, ssh_url: String, stargazers_count: i32, stargazers_url: String, statuses_url: String, subscribers_url: String, subscription_url: String, svn_url: String, tags_url: String, teams_url: String, topics: Vec<String>, trees_url: String, updated_at: String, url: String, visibility: Visibility, watchers: i32, watchers_count: i32) -> Repository4 {
        Repository4 {
            allow_auto_merge: None,
            allow_forking: None,
            allow_merge_commit: None,
            allow_rebase_merge: None,
            allow_squash_merge: None,
            allow_update_branch: None,
            archive_url,
            archived,
            assignees_url,
            blobs_url,
            branches_url,
            clone_url,
            collaborators_url,
            comments_url,
            commits_url,
            compare_url,
            contents_url,
            contributors_url,
            created_at: Box::new(created_at),
            default_branch,
            delete_branch_on_merge: None,
            deployments_url,
            description,
            disabled: None,
            downloads_url,
            events_url,
            fork,
            forks,
            forks_count,
            forks_url,
            full_name,
            git_commits_url,
            git_refs_url,
            git_tags_url,
            git_url,
            has_downloads,
            has_issues,
            has_pages,
            has_projects,
            has_wiki,
            has_discussions,
            homepage,
            hooks_url,
            html_url,
            id,
            is_template: None,
            issue_comment_url,
            issue_events_url,
            issues_url,
            keys_url,
            labels_url,
            language,
            languages_url,
            license: if let Some(x) = license {Some(Box::new(x))} else {None},
            master_branch: None,
            merge_commit_message: None,
            merge_commit_title: None,
            merges_url,
            milestones_url,
            mirror_url,
            name,
            node_id,
            notifications_url,
            open_issues,
            open_issues_count,
            organization: None,
            owner: if let Some(x) = owner {Some(Box::new(x))} else {None},
            permissions: None,
            private,
            public: None,
            pulls_url,
            pushed_at: if let Some(x) = pushed_at {Some(Box::new(x))} else {None},
            releases_url,
            role_name: None,
            size,
            squash_merge_commit_message: None,
            squash_merge_commit_title: None,
            ssh_url,
            stargazers: None,
            stargazers_count,
            stargazers_url,
            statuses_url,
            subscribers_url,
            subscription_url,
            svn_url,
            tags_url,
            teams_url,
            topics,
            trees_url,
            updated_at,
            url,
            use_squash_pr_title_as_default: None,
            visibility,
            watchers,
            watchers_count,
            web_commit_signoff_required: None,
        }
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
/// The default value for a merge commit title.  - `PR_TITLE` - default to the pull request's title. - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name).
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
/// The default value for a squash merge commit title:  - `PR_TITLE` - default to the pull request's title. - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).
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
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "internal")]
    Internal,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Self::Public
    }
}

