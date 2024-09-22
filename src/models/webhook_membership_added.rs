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
pub struct WebhookMembershipAdded {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "member", deserialize_with = "Option::deserialize")]
    pub member: Option<Box<models::WebhooksUser>>,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimpleWebhooks>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::RepositoryWebhooks>>,
    /// The scope of the membership. Currently, can only be `team`.
    #[serde(rename = "scope")]
    pub scope: Scope,
    #[serde(rename = "sender", deserialize_with = "Option::deserialize")]
    pub sender: Option<Box<models::User>>,
    #[serde(rename = "team")]
    pub team: Box<models::WebhooksTeam>,
}

impl WebhookMembershipAdded {
    pub fn new(action: Action, member: Option<models::WebhooksUser>, organization: models::OrganizationSimpleWebhooks, scope: Scope, sender: Option<models::User>, team: models::WebhooksTeam) -> WebhookMembershipAdded {
        WebhookMembershipAdded {
            action,
            enterprise: None,
            installation: None,
            member: if let Some(x) = member {Some(Box::new(x))} else {None},
            organization: Box::new(organization),
            repository: None,
            scope,
            sender: if let Some(x) = sender {Some(Box::new(x))} else {None},
            team: Box::new(team),
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
/// The scope of the membership. Currently, can only be `team`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "team")]
    Team,
}

impl Default for Scope {
    fn default() -> Scope {
        Self::Team
    }
}

