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

/// App1 : GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct App1 {
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<String>,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// The list of events for the GitHub app
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Events>>,
    #[serde(rename = "external_url", deserialize_with = "Option::deserialize")]
    pub external_url: Option<String>,
    #[serde(rename = "html_url")]
    pub html_url: String,
    /// Unique identifier of the GitHub app
    #[serde(rename = "id", deserialize_with = "Option::deserialize")]
    pub id: Option<i32>,
    /// The name of the GitHub app
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "owner", deserialize_with = "Option::deserialize")]
    pub owner: Option<Box<models::User>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<models::App1Permissions>>,
    /// The slug name of the GitHub app
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
}

impl App1 {
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    pub fn new(created_at: Option<String>, description: Option<String>, external_url: Option<String>, html_url: String, id: Option<i32>, name: String, node_id: String, owner: Option<models::User>, updated_at: Option<String>) -> App1 {
        App1 {
            created_at,
            description,
            events: None,
            external_url,
            html_url,
            id,
            name,
            node_id,
            owner: if let Some(x) = owner {Some(Box::new(x))} else {None},
            permissions: None,
            slug: None,
            updated_at,
        }
    }
}
/// The list of events for the GitHub app
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Events {
    #[serde(rename = "branch_protection_rule")]
    BranchProtectionRule,
    #[serde(rename = "check_run")]
    CheckRun,
    #[serde(rename = "check_suite")]
    CheckSuite,
    #[serde(rename = "code_scanning_alert")]
    CodeScanningAlert,
    #[serde(rename = "commit_comment")]
    CommitComment,
    #[serde(rename = "content_reference")]
    ContentReference,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "deployment")]
    Deployment,
    #[serde(rename = "deployment_review")]
    DeploymentReview,
    #[serde(rename = "deployment_status")]
    DeploymentStatus,
    #[serde(rename = "deploy_key")]
    DeployKey,
    #[serde(rename = "discussion")]
    Discussion,
    #[serde(rename = "discussion_comment")]
    DiscussionComment,
    #[serde(rename = "fork")]
    Fork,
    #[serde(rename = "gollum")]
    Gollum,
    #[serde(rename = "issues")]
    Issues,
    #[serde(rename = "issue_comment")]
    IssueComment,
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "membership")]
    Membership,
    #[serde(rename = "milestone")]
    Milestone,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "org_block")]
    OrgBlock,
    #[serde(rename = "page_build")]
    PageBuild,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "project_card")]
    ProjectCard,
    #[serde(rename = "project_column")]
    ProjectColumn,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "pull_request")]
    PullRequest,
    #[serde(rename = "pull_request_review")]
    PullRequestReview,
    #[serde(rename = "pull_request_review_comment")]
    PullRequestReviewComment,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "registry_package")]
    RegistryPackage,
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "repository")]
    Repository,
    #[serde(rename = "repository_dispatch")]
    RepositoryDispatch,
    #[serde(rename = "secret_scanning_alert")]
    SecretScanningAlert,
    #[serde(rename = "star")]
    Star,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "team")]
    Team,
    #[serde(rename = "team_add")]
    TeamAdd,
    #[serde(rename = "watch")]
    Watch,
    #[serde(rename = "workflow_dispatch")]
    WorkflowDispatch,
    #[serde(rename = "workflow_run")]
    WorkflowRun,
}

impl Default for Events {
    fn default() -> Events {
        Self::BranchProtectionRule
    }
}

