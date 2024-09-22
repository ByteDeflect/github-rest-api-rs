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
pub struct OrgsReviewPatGrantRequestsInBulkRequest {
    /// Unique identifiers of the requests for access via fine-grained personal access token. Must be formed of between 1 and 100 `pat_request_id` values.
    #[serde(rename = "pat_request_ids", skip_serializing_if = "Option::is_none")]
    pub pat_request_ids: Option<Vec<i32>>,
    /// Action to apply to the requests.
    #[serde(rename = "action")]
    pub action: Action,
    /// Reason for approving or denying the requests. Max 1024 characters.
    #[serde(rename = "reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
}

impl OrgsReviewPatGrantRequestsInBulkRequest {
    pub fn new(action: Action) -> OrgsReviewPatGrantRequestsInBulkRequest {
        OrgsReviewPatGrantRequestsInBulkRequest {
            pat_request_ids: None,
            action,
            reason: None,
        }
    }
}
/// Action to apply to the requests.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "approve")]
    Approve,
    #[serde(rename = "deny")]
    Deny,
}

impl Default for Action {
    fn default() -> Action {
        Self::Approve
    }
}

