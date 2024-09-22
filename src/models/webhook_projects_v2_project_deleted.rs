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
pub struct WebhookProjectsV2ProjectDeleted {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimpleWebhooks>,
    #[serde(rename = "projects_v2")]
    pub projects_v2: Box<models::ProjectsV2>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookProjectsV2ProjectDeleted {
    pub fn new(action: Action, organization: models::OrganizationSimpleWebhooks, projects_v2: models::ProjectsV2, sender: models::SimpleUserWebhooks) -> WebhookProjectsV2ProjectDeleted {
        WebhookProjectsV2ProjectDeleted {
            action,
            installation: None,
            organization: Box::new(organization),
            projects_v2: Box::new(projects_v2),
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

