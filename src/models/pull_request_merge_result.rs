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

/// PullRequestMergeResult : Pull Request Merge Result
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMergeResult {
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "merged")]
    pub merged: bool,
    #[serde(rename = "message")]
    pub message: String,
}

impl PullRequestMergeResult {
    /// Pull Request Merge Result
    pub fn new(sha: String, merged: bool, message: String) -> PullRequestMergeResult {
        PullRequestMergeResult {
            sha,
            merged,
            message,
        }
    }
}

