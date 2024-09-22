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
pub struct FileExtensionRestrictionParameters {
    /// The file extensions that are restricted from being pushed to the commit graph.
    #[serde(rename = "restricted_file_extensions")]
    pub restricted_file_extensions: Vec<String>,
}

impl FileExtensionRestrictionParameters {
    pub fn new(restricted_file_extensions: Vec<String>) -> FileExtensionRestrictionParameters {
        FileExtensionRestrictionParameters {
            restricted_file_extensions,
        }
    }
}

