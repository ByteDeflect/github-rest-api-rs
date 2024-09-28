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
pub struct WebhookPullRequestAutoMergeEnabled {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "pull_request")]
    pub pull_request: Box<models::PullRequest2>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUser>,
}

impl WebhookPullRequestAutoMergeEnabled {
    pub fn new(action: Action, number: i32, pull_request: models::PullRequest2, repository: models::RepositoryWebhooks, sender: models::SimpleUser) -> WebhookPullRequestAutoMergeEnabled {
        WebhookPullRequestAutoMergeEnabled {
            action,
            enterprise: None,
            installation: None,
            number,
            organization: None,
            pull_request: Box::new(pull_request),
            reason: None,
            repository: Box::new(repository),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "auto_merge_enabled")]
    AutoMergeEnabled,
}

impl Default for Action {
    fn default() -> Action {
        Self::AutoMergeEnabled
    }
}

