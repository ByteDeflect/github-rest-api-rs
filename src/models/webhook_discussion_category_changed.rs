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
pub struct WebhookDiscussionCategoryChanged {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "changes")]
    pub changes: Box<models::WebhookDiscussionCategoryChangedChanges>,
    #[serde(rename = "discussion")]
    pub discussion: Box<models::Discussion>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookDiscussionCategoryChanged {
    pub fn new(action: Action, changes: models::WebhookDiscussionCategoryChangedChanges, discussion: models::Discussion, repository: models::RepositoryWebhooks, sender: models::SimpleUserWebhooks) -> WebhookDiscussionCategoryChanged {
        WebhookDiscussionCategoryChanged {
            action,
            changes: Box::new(changes),
            discussion: Box::new(discussion),
            enterprise: None,
            installation: None,
            organization: None,
            repository: Box::new(repository),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "category_changed")]
    CategoryChanged,
}

impl Default for Action {
    fn default() -> Action {
        Self::CategoryChanged
    }
}

