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
pub struct WebhookInstallationTargetRenamed {
    #[serde(rename = "account")]
    pub account: Box<models::WebhookInstallationTargetRenamedAccount>,
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "changes")]
    pub changes: Box<models::WebhookInstallationTargetRenamedChanges>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation")]
    pub installation: Box<models::SimpleInstallation>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::RepositoryWebhooks>>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<models::SimpleUserWebhooks>>,
    #[serde(rename = "target_type")]
    pub target_type: String,
}

impl WebhookInstallationTargetRenamed {
    pub fn new(account: models::WebhookInstallationTargetRenamedAccount, action: Action, changes: models::WebhookInstallationTargetRenamedChanges, installation: models::SimpleInstallation, target_type: String) -> WebhookInstallationTargetRenamed {
        WebhookInstallationTargetRenamed {
            account: Box::new(account),
            action,
            changes: Box::new(changes),
            enterprise: None,
            installation: Box::new(installation),
            organization: None,
            repository: None,
            sender: None,
            target_type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "renamed")]
    Renamed,
}

impl Default for Action {
    fn default() -> Action {
        Self::Renamed
    }
}

