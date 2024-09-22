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
pub struct FileCommitContent {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "sha", skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "git_url", skip_serializing_if = "Option::is_none")]
    pub git_url: Option<String>,
    #[serde(rename = "download_url", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    pub _links: Option<Box<models::FileCommitContentLinks>>,
}

impl FileCommitContent {
    pub fn new() -> FileCommitContent {
        FileCommitContent {
            name: None,
            path: None,
            sha: None,
            size: None,
            url: None,
            html_url: None,
            git_url: None,
            download_url: None,
            r#type: None,
            _links: None,
        }
    }
}

