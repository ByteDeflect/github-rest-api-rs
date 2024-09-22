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
pub struct WebhookProjectsV2ProjectEdited {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "changes")]
    pub changes: Box<models::WebhookProjectsV2ProjectEditedChanges>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimpleWebhooks>,
    #[serde(rename = "projects_v2")]
    pub projects_v2: Box<models::ProjectsV2>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookProjectsV2ProjectEdited {
    pub fn new(action: Action, changes: models::WebhookProjectsV2ProjectEditedChanges, organization: models::OrganizationSimpleWebhooks, projects_v2: models::ProjectsV2, sender: models::SimpleUserWebhooks) -> WebhookProjectsV2ProjectEdited {
        WebhookProjectsV2ProjectEdited {
            action,
            changes: Box::new(changes),
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
    #[serde(rename = "edited")]
    Edited,
}

impl Default for Action {
    fn default() -> Action {
        Self::Edited
    }
}

