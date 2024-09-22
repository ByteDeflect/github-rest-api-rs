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
pub struct WebhookRepositoryRulesetEditedChangesRules {
    #[serde(rename = "added", skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<models::RepositoryRule>>,
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Vec<models::RepositoryRule>>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<models::WebhookRepositoryRulesetEditedChangesRulesUpdatedInner>>,
}

impl WebhookRepositoryRulesetEditedChangesRules {
    pub fn new() -> WebhookRepositoryRulesetEditedChangesRules {
        WebhookRepositoryRulesetEditedChangesRules {
            added: None,
            deleted: None,
            updated: None,
        }
    }
}

