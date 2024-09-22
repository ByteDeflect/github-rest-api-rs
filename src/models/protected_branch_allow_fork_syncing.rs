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

/// ProtectedBranchAllowForkSyncing : Whether users can pull changes from upstream when the branch is locked. Set to `true` to allow fork syncing. Set to `false` to prevent fork syncing.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectedBranchAllowForkSyncing {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl ProtectedBranchAllowForkSyncing {
    /// Whether users can pull changes from upstream when the branch is locked. Set to `true` to allow fork syncing. Set to `false` to prevent fork syncing.
    pub fn new() -> ProtectedBranchAllowForkSyncing {
        ProtectedBranchAllowForkSyncing {
            enabled: None,
        }
    }
}

