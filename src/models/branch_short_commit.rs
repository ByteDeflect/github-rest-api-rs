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
pub struct BranchShortCommit {
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl BranchShortCommit {
    pub fn new(sha: String, url: String) -> BranchShortCommit {
        BranchShortCommit {
            sha,
            url,
        }
    }
}

