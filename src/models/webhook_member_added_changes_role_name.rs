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

/// WebhookMemberAddedChangesRoleName : The role assigned to the collaborator.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookMemberAddedChangesRoleName {
    #[serde(rename = "to")]
    pub to: String,
}

impl WebhookMemberAddedChangesRoleName {
    /// The role assigned to the collaborator.
    pub fn new(to: String) -> WebhookMemberAddedChangesRoleName {
        WebhookMemberAddedChangesRoleName {
            to,
        }
    }
}

