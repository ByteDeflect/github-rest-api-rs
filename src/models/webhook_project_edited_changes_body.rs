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
pub struct WebhookProjectEditedChangesBody {
    /// The previous version of the body if the action was `edited`.
    #[serde(rename = "from")]
    pub from: String,
}

impl WebhookProjectEditedChangesBody {
    pub fn new(from: String) -> WebhookProjectEditedChangesBody {
        WebhookProjectEditedChangesBody {
            from,
        }
    }
}

