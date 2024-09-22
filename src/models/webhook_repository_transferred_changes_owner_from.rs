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
pub struct WebhookRepositoryTransferredChangesOwnerFrom {
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::Organization>>,
    #[serde(rename = "user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user: Option<Option<Box<models::User1>>>,
}

impl WebhookRepositoryTransferredChangesOwnerFrom {
    pub fn new() -> WebhookRepositoryTransferredChangesOwnerFrom {
        WebhookRepositoryTransferredChangesOwnerFrom {
            organization: None,
            user: None,
        }
    }
}

