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

/// OrganizationProgrammaticAccessGrantRequestPermissions : Permissions requested, categorized by type of permission.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationProgrammaticAccessGrantRequestPermissions {
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "other", skip_serializing_if = "Option::is_none")]
    pub other: Option<std::collections::HashMap<String, String>>,
}

impl OrganizationProgrammaticAccessGrantRequestPermissions {
    /// Permissions requested, categorized by type of permission.
    pub fn new() -> OrganizationProgrammaticAccessGrantRequestPermissions {
        OrganizationProgrammaticAccessGrantRequestPermissions {
            organization: None,
            repository: None,
            other: None,
        }
    }
}

