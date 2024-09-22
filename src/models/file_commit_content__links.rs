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
pub struct FileCommitContentLinks {
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    #[serde(rename = "git", skip_serializing_if = "Option::is_none")]
    pub git: Option<String>,
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
}

impl FileCommitContentLinks {
    pub fn new() -> FileCommitContentLinks {
        FileCommitContentLinks {
            param_self: None,
            git: None,
            html: None,
        }
    }
}

