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
pub struct WebhookProjectsV2ItemDeleted {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimpleWebhooks>,
    #[serde(rename = "projects_v2_item")]
    pub projects_v2_item: Box<models::ProjectsV2Item>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookProjectsV2ItemDeleted {
    pub fn new(action: Action, organization: models::OrganizationSimpleWebhooks, projects_v2_item: models::ProjectsV2Item, sender: models::SimpleUserWebhooks) -> WebhookProjectsV2ItemDeleted {
        WebhookProjectsV2ItemDeleted {
            action,
            installation: None,
            organization: Box::new(organization),
            projects_v2_item: Box::new(projects_v2_item),
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

