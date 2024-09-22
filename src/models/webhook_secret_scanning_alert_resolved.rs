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
pub struct WebhookSecretScanningAlertResolved {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "alert")]
    pub alert: Box<models::SecretScanningAlertWebhook>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<models::SimpleUserWebhooks>>,
}

impl WebhookSecretScanningAlertResolved {
    pub fn new(action: Action, alert: models::SecretScanningAlertWebhook, repository: models::RepositoryWebhooks) -> WebhookSecretScanningAlertResolved {
        WebhookSecretScanningAlertResolved {
            action,
            alert: Box::new(alert),
            enterprise: None,
            installation: None,
            organization: None,
            repository: Box::new(repository),
            sender: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "resolved")]
    Resolved,
}

impl Default for Action {
    fn default() -> Action {
        Self::Resolved
    }
}

