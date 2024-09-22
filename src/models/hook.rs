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

/// Hook : Webhooks for repositories.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hook {
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier of the webhook.
    #[serde(rename = "id")]
    pub id: i32,
    /// The name of a valid service, use 'web' for a webhook.
    #[serde(rename = "name")]
    pub name: String,
    /// Determines whether the hook is actually triggered on pushes.
    #[serde(rename = "active")]
    pub active: bool,
    /// Determines what events the hook is triggered for. Default: ['push'].
    #[serde(rename = "events")]
    pub events: Vec<String>,
    #[serde(rename = "config")]
    pub config: Box<models::WebhookConfig>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "test_url")]
    pub test_url: String,
    #[serde(rename = "ping_url")]
    pub ping_url: String,
    #[serde(rename = "deliveries_url", skip_serializing_if = "Option::is_none")]
    pub deliveries_url: Option<String>,
    #[serde(rename = "last_response")]
    pub last_response: Box<models::HookResponse>,
}

impl Hook {
    /// Webhooks for repositories.
    pub fn new(r#type: String, id: i32, name: String, active: bool, events: Vec<String>, config: models::WebhookConfig, updated_at: String, created_at: String, url: String, test_url: String, ping_url: String, last_response: models::HookResponse) -> Hook {
        Hook {
            r#type,
            id,
            name,
            active,
            events,
            config: Box::new(config),
            updated_at,
            created_at,
            url,
            test_url,
            ping_url,
            deliveries_url: None,
            last_response: Box::new(last_response),
        }
    }
}

