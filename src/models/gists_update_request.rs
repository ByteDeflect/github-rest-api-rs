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
pub struct GistsUpdateRequest {
    /// The description of the gist.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The gist files to be updated, renamed, or deleted. Each `key` must match the current filename (including extension) of the targeted gist file. For example: `hello.py`.  To delete a file, set the whole file to null. For example: `hello.py : null`. The file will also be deleted if the specified object does not contain at least one of `content` or `filename`.
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<std::collections::HashMap<String, models::GistsUpdateRequestFilesValue>>,
}

impl GistsUpdateRequest {
    pub fn new() -> GistsUpdateRequest {
        GistsUpdateRequest {
            description: None,
            files: None,
        }
    }
}

