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
pub struct WebhookSecurityAdvisoryPublished {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::RepositoryWebhooks>>,
    #[serde(rename = "security_advisory")]
    pub security_advisory: Box<models::WebhooksSecurityAdvisory>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<models::SimpleUserWebhooks>>,
}

impl WebhookSecurityAdvisoryPublished {
    pub fn new(action: Action, security_advisory: models::WebhooksSecurityAdvisory) -> WebhookSecurityAdvisoryPublished {
        WebhookSecurityAdvisoryPublished {
            action,
            enterprise: None,
            installation: None,
            organization: None,
            repository: None,
            security_advisory: Box::new(security_advisory),
            sender: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "published")]
    Published,
}

impl Default for Action {
    fn default() -> Action {
        Self::Published
    }
}

