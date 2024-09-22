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
pub struct ActivitySetRepoSubscriptionRequest {
    /// Determines if notifications should be received from this repository.
    #[serde(rename = "subscribed", skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    /// Determines if all notifications should be blocked from this repository.
    #[serde(rename = "ignored", skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
}

impl ActivitySetRepoSubscriptionRequest {
    pub fn new() -> ActivitySetRepoSubscriptionRequest {
        ActivitySetRepoSubscriptionRequest {
            subscribed: None,
            ignored: None,
        }
    }
}

