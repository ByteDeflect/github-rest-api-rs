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
pub struct WebhookInstallationDeleted {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation")]
    pub installation: Box<models::Installation>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    /// An array of repository objects that the installation can access.
    #[serde(rename = "repositories", skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<models::WebhooksRepositoriesInner>>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::RepositoryWebhooks>>,
    #[serde(rename = "requester", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub requester: Option<Option<serde_json::Value>>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUser>,
}

impl WebhookInstallationDeleted {
    pub fn new(action: Action, installation: models::Installation, sender: models::SimpleUser) -> WebhookInstallationDeleted {
        WebhookInstallationDeleted {
            action,
            enterprise: None,
            installation: Box::new(installation),
            organization: None,
            repositories: None,
            repository: None,
            requester: None,
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for Action {
    fn default() -> Action {
        Self::Deleted
    }
}

