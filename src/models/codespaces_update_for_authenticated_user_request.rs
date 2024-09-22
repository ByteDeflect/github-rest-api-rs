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
pub struct CodespacesUpdateForAuthenticatedUserRequest {
    /// A valid machine to transition this codespace to.
    #[serde(rename = "machine", skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,
    /// Display name for this codespace
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Recently opened folders inside the codespace. It is currently used by the clients to determine the folder path to load the codespace in.
    #[serde(rename = "recent_folders", skip_serializing_if = "Option::is_none")]
    pub recent_folders: Option<Vec<String>>,
}

impl CodespacesUpdateForAuthenticatedUserRequest {
    pub fn new() -> CodespacesUpdateForAuthenticatedUserRequest {
        CodespacesUpdateForAuthenticatedUserRequest {
            machine: None,
            display_name: None,
            recent_folders: None,
        }
    }
}

