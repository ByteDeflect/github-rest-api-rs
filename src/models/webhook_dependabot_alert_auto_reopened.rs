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
pub struct WebhookDependabotAlertAutoReopened {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "alert")]
    pub alert: Box<models::DependabotAlert>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUser>,
}

impl WebhookDependabotAlertAutoReopened {
    pub fn new(action: Action, alert: models::DependabotAlert, repository: models::RepositoryWebhooks, sender: models::SimpleUser) -> WebhookDependabotAlertAutoReopened {
        WebhookDependabotAlertAutoReopened {
            action,
            alert: Box::new(alert),
            installation: None,
            organization: None,
            enterprise: None,
            repository: Box::new(repository),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "auto_reopened")]
    AutoReopened,
}

impl Default for Action {
    fn default() -> Action {
        Self::AutoReopened
    }
}

