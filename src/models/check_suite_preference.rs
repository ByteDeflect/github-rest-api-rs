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

/// CheckSuitePreference : Check suite configuration preferences for a repository.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckSuitePreference {
    #[serde(rename = "preferences")]
    pub preferences: Box<models::CheckSuitePreferencePreferences>,
    #[serde(rename = "repository")]
    pub repository: Box<models::MinimalRepository>,
}

impl CheckSuitePreference {
    /// Check suite configuration preferences for a repository.
    pub fn new(preferences: models::CheckSuitePreferencePreferences, repository: models::MinimalRepository) -> CheckSuitePreference {
        CheckSuitePreference {
            preferences: Box::new(preferences),
            repository: Box::new(repository),
        }
    }
}

