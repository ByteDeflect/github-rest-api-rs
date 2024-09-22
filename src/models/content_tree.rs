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

/// ContentTree : Content Tree
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentTree {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "size")]
    pub size: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "git_url", deserialize_with = "Option::deserialize")]
    pub git_url: Option<String>,
    #[serde(rename = "html_url", deserialize_with = "Option::deserialize")]
    pub html_url: Option<String>,
    #[serde(rename = "download_url", deserialize_with = "Option::deserialize")]
    pub download_url: Option<String>,
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<models::ContentTreeEntriesInner>>,
    #[serde(rename = "_links")]
    pub _links: Box<models::ContentTreeEntriesInnerLinks>,
}

impl ContentTree {
    /// Content Tree
    pub fn new(r#type: String, size: i32, name: String, path: String, sha: String, url: String, git_url: Option<String>, html_url: Option<String>, download_url: Option<String>, _links: models::ContentTreeEntriesInnerLinks) -> ContentTree {
        ContentTree {
            r#type,
            size,
            name,
            path,
            sha,
            url,
            git_url,
            html_url,
            download_url,
            entries: None,
            _links: Box::new(_links),
        }
    }
}

