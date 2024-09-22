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
pub struct GistsUpdateRequestFilesValue {
    /// The new content of the file.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The new filename for the file.
    #[serde(rename = "filename", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filename: Option<Option<String>>,
}

impl GistsUpdateRequestFilesValue {
    pub fn new() -> GistsUpdateRequestFilesValue {
        GistsUpdateRequestFilesValue {
            content: None,
            filename: None,
        }
    }
}

