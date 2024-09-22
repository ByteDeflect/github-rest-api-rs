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
pub struct DockerMetadata {
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<String>>,
}

impl DockerMetadata {
    pub fn new() -> DockerMetadata {
        DockerMetadata {
            tag: None,
        }
    }
}

