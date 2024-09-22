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

/// BranchProtectionLockBranch : Whether to set the branch as read-only. If this is true, users will not be able to push to the branch.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BranchProtectionLockBranch {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl BranchProtectionLockBranch {
    /// Whether to set the branch as read-only. If this is true, users will not be able to push to the branch.
    pub fn new() -> BranchProtectionLockBranch {
        BranchProtectionLockBranch {
            enabled: None,
        }
    }
}

