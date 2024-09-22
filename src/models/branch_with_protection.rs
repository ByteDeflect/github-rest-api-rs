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

/// BranchWithProtection : Branch With Protection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BranchWithProtection {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "commit")]
    pub commit: Box<models::Commit>,
    #[serde(rename = "_links")]
    pub _links: Box<models::BranchWithProtectionLinks>,
    #[serde(rename = "protected")]
    pub protected: bool,
    #[serde(rename = "protection")]
    pub protection: Box<models::BranchProtection>,
    #[serde(rename = "protection_url")]
    pub protection_url: String,
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "required_approving_review_count", skip_serializing_if = "Option::is_none")]
    pub required_approving_review_count: Option<i32>,
}

impl BranchWithProtection {
    /// Branch With Protection
    pub fn new(name: String, commit: models::Commit, _links: models::BranchWithProtectionLinks, protected: bool, protection: models::BranchProtection, protection_url: String) -> BranchWithProtection {
        BranchWithProtection {
            name,
            commit: Box::new(commit),
            _links: Box::new(_links),
            protected,
            protection: Box::new(protection),
            protection_url,
            pattern: None,
            required_approving_review_count: None,
        }
    }
}

