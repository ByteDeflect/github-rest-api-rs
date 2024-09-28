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
pub struct WebhookBranchProtectionRuleEdited {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Box<models::WebhookBranchProtectionRuleEditedChanges>>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "rule")]
    pub rule: Box<models::WebhooksRule>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUser>,
}

impl WebhookBranchProtectionRuleEdited {
    pub fn new(action: Action, repository: models::RepositoryWebhooks, rule: models::WebhooksRule, sender: models::SimpleUser) -> WebhookBranchProtectionRuleEdited {
        WebhookBranchProtectionRuleEdited {
            action,
            changes: None,
            enterprise: None,
            installation: None,
            organization: None,
            repository: Box::new(repository),
            rule: Box::new(rule),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "edited")]
    Edited,
}

impl Default for Action {
    fn default() -> Action {
        Self::Edited
    }
}

