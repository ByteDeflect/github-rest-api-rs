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
pub struct NullableScopedInstallation {
    #[serde(rename = "permissions")]
    pub permissions: Box<models::AppPermissions>,
    /// Describe whether all repositories have been selected or there's a selection involved
    #[serde(rename = "repository_selection")]
    pub repository_selection: RepositorySelection,
    #[serde(rename = "single_file_name", deserialize_with = "Option::deserialize")]
    pub single_file_name: Option<String>,
    #[serde(rename = "has_multiple_single_files", skip_serializing_if = "Option::is_none")]
    pub has_multiple_single_files: Option<bool>,
    #[serde(rename = "single_file_paths", skip_serializing_if = "Option::is_none")]
    pub single_file_paths: Option<Vec<String>>,
    #[serde(rename = "repositories_url")]
    pub repositories_url: String,
    #[serde(rename = "account")]
    pub account: Box<models::SimpleUser>,
}

impl NullableScopedInstallation {
    pub fn new(permissions: models::AppPermissions, repository_selection: RepositorySelection, single_file_name: Option<String>, repositories_url: String, account: models::SimpleUser) -> NullableScopedInstallation {
        NullableScopedInstallation {
            permissions: Box::new(permissions),
            repository_selection,
            single_file_name,
            has_multiple_single_files: None,
            single_file_paths: None,
            repositories_url,
            account: Box::new(account),
        }
    }
}
/// Describe whether all repositories have been selected or there's a selection involved
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RepositorySelection {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "selected")]
    Selected,
}

impl Default for RepositorySelection {
    fn default() -> RepositorySelection {
        Self::All
    }
}

