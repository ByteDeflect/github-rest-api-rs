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
pub struct ActionsOrganizationPermissions {
    #[serde(rename = "enabled_repositories")]
    pub enabled_repositories: models::EnabledRepositories,
    /// The API URL to use to get or set the selected repositories that are allowed to run GitHub Actions, when `enabled_repositories` is set to `selected`.
    #[serde(rename = "selected_repositories_url", skip_serializing_if = "Option::is_none")]
    pub selected_repositories_url: Option<String>,
    #[serde(rename = "allowed_actions", skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<models::AllowedActions>,
    /// The API URL to use to get or set the actions and reusable workflows that are allowed to run, when `allowed_actions` is set to `selected`.
    #[serde(rename = "selected_actions_url", skip_serializing_if = "Option::is_none")]
    pub selected_actions_url: Option<String>,
}

impl ActionsOrganizationPermissions {
    pub fn new(enabled_repositories: models::EnabledRepositories) -> ActionsOrganizationPermissions {
        ActionsOrganizationPermissions {
            enabled_repositories,
            selected_repositories_url: None,
            allowed_actions: None,
            selected_actions_url: None,
        }
    }
}

