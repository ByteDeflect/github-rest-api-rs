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
pub struct CheckSuitePreferencePreferences {
    #[serde(rename = "auto_trigger_checks", skip_serializing_if = "Option::is_none")]
    pub auto_trigger_checks: Option<Vec<models::CheckSuitePreferencePreferencesAutoTriggerChecksInner>>,
}

impl CheckSuitePreferencePreferences {
    pub fn new() -> CheckSuitePreferencePreferences {
        CheckSuitePreferencePreferences {
            auto_trigger_checks: None,
        }
    }
}

