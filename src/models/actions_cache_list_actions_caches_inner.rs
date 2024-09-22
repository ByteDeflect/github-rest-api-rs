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
pub struct ActionsCacheListActionsCachesInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "last_accessed_at", skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "size_in_bytes", skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i32>,
}

impl ActionsCacheListActionsCachesInner {
    pub fn new() -> ActionsCacheListActionsCachesInner {
        ActionsCacheListActionsCachesInner {
            id: None,
            r#ref: None,
            key: None,
            version: None,
            last_accessed_at: None,
            created_at: None,
            size_in_bytes: None,
        }
    }
}

