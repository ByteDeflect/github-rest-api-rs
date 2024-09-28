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
pub struct WebhookRepositoryDispatchSample {
    /// The `event_type` that was specified in the `POST /repos/{owner}/{repo}/dispatches` request body.
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "branch")]
    pub branch: String,
    /// The `client_payload` that was specified in the `POST /repos/{owner}/{repo}/dispatches` request body.
    #[serde(rename = "client_payload", deserialize_with = "Option::deserialize")]
    pub client_payload: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation")]
    pub installation: Box<models::SimpleInstallation>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUser>,
}

impl WebhookRepositoryDispatchSample {
    pub fn new(action: String, branch: String, client_payload: Option<std::collections::HashMap<String, serde_json::Value>>, installation: models::SimpleInstallation, repository: models::RepositoryWebhooks, sender: models::SimpleUser) -> WebhookRepositoryDispatchSample {
        WebhookRepositoryDispatchSample {
            action,
            branch,
            client_payload,
            enterprise: None,
            installation: Box::new(installation),
            organization: None,
            repository: Box::new(repository),
            sender: Box::new(sender),
        }
    }
}

