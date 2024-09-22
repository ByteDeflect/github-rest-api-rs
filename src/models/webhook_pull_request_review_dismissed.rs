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
pub struct WebhookPullRequestReviewDismissed {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "pull_request")]
    pub pull_request: Box<models::SimplePullRequest>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "review")]
    pub review: Box<models::WebhookPullRequestReviewDismissedReview>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookPullRequestReviewDismissed {
    pub fn new(action: Action, pull_request: models::SimplePullRequest, repository: models::RepositoryWebhooks, review: models::WebhookPullRequestReviewDismissedReview, sender: models::SimpleUserWebhooks) -> WebhookPullRequestReviewDismissed {
        WebhookPullRequestReviewDismissed {
            action,
            enterprise: None,
            installation: None,
            organization: None,
            pull_request: Box::new(pull_request),
            repository: Box::new(repository),
            review: Box::new(review),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "dismissed")]
    Dismissed,
}

impl Default for Action {
    fn default() -> Action {
        Self::Dismissed
    }
}

