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
pub struct WebhookIssuesUnassigned {
    /// The action that was performed.
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "assignee", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Option<Box<models::WebhooksUserMannequin>>>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "issue")]
    pub issue: Box<models::WebhooksIssue>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookIssuesUnassigned {
    pub fn new(action: Action, issue: models::WebhooksIssue, repository: models::RepositoryWebhooks, sender: models::SimpleUserWebhooks) -> WebhookIssuesUnassigned {
        WebhookIssuesUnassigned {
            action,
            assignee: None,
            enterprise: None,
            installation: None,
            issue: Box::new(issue),
            organization: None,
            repository: Box::new(repository),
            sender: Box::new(sender),
        }
    }
}
/// The action that was performed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "unassigned")]
    Unassigned,
}

impl Default for Action {
    fn default() -> Action {
        Self::Unassigned
    }
}

