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
pub struct MigrationsMapCommitAuthorRequest {
    /// The new Git author email.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The new Git author name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl MigrationsMapCommitAuthorRequest {
    pub fn new() -> MigrationsMapCommitAuthorRequest {
        MigrationsMapCommitAuthorRequest {
            email: None,
            name: None,
        }
    }
}

