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

/// PersonalAccessTokenRequestPermissionsResult : Permissions requested, categorized by type of permission. This field incorporates `permissions_added` and `permissions_upgraded`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalAccessTokenRequestPermissionsResult {
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "other", skip_serializing_if = "Option::is_none")]
    pub other: Option<std::collections::HashMap<String, String>>,
}

impl PersonalAccessTokenRequestPermissionsResult {
    /// Permissions requested, categorized by type of permission. This field incorporates `permissions_added` and `permissions_upgraded`.
    pub fn new() -> PersonalAccessTokenRequestPermissionsResult {
        PersonalAccessTokenRequestPermissionsResult {
            organization: None,
            repository: None,
            other: None,
        }
    }
}

