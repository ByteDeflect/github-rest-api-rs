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

/// AllowedActions : The permissions policy that controls the actions and reusable workflows that are allowed to run.
/// The permissions policy that controls the actions and reusable workflows that are allowed to run.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AllowedActions {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "local_only")]
    LocalOnly,
    #[serde(rename = "selected")]
    Selected,

}

impl std::fmt::Display for AllowedActions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::LocalOnly => write!(f, "local_only"),
            Self::Selected => write!(f, "selected"),
        }
    }
}

impl Default for AllowedActions {
    fn default() -> AllowedActions {
        Self::All
    }
}

