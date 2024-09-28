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
pub struct WebhookMemberAdded {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Box<models::WebhookMemberAddedChanges>>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "member", deserialize_with = "Option::deserialize")]
    pub member: Option<Box<models::WebhooksUser>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUser>,
}

impl WebhookMemberAdded {
    pub fn new(action: Action, member: Option<models::WebhooksUser>, repository: models::RepositoryWebhooks, sender: models::SimpleUser) -> WebhookMemberAdded {
        WebhookMemberAdded {
            action,
            changes: None,
            enterprise: None,
            installation: None,
            member: if let Some(x) = member {Some(Box::new(x))} else {None},
            organization: None,
            repository: Box::new(repository),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "added")]
    Added,
}

impl Default for Action {
    fn default() -> Action {
        Self::Added
    }
}

