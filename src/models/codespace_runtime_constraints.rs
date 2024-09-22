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
pub struct CodespaceRuntimeConstraints {
    /// The privacy settings a user can select from when forwarding a port.
    #[serde(rename = "allowed_port_privacy_settings", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allowed_port_privacy_settings: Option<Option<Vec<String>>>,
}

impl CodespaceRuntimeConstraints {
    pub fn new() -> CodespaceRuntimeConstraints {
        CodespaceRuntimeConstraints {
            allowed_port_privacy_settings: None,
        }
    }
}

