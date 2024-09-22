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
pub struct SecretScanningAlertWebhook {
    /// The security alert number.
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    /// The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The time that the alert was last updated in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "updated_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Option<String>>,
    /// The REST API URL of the alert resource.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The GitHub URL of the alert resource.
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    /// The REST API URL of the code locations for this alert.
    #[serde(rename = "locations_url", skip_serializing_if = "Option::is_none")]
    pub locations_url: Option<String>,
    #[serde(rename = "resolution", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Option<models::SecretScanningAlertResolutionWebhook>>,
    /// The time that the alert was resolved in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "resolved_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<Option<String>>,
    #[serde(rename = "resolved_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<Option<Box<models::NullableSimpleUser>>>,
    /// An optional comment to resolve an alert.
    #[serde(rename = "resolution_comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub resolution_comment: Option<Option<String>>,
    /// The type of secret that secret scanning detected.
    #[serde(rename = "secret_type", skip_serializing_if = "Option::is_none")]
    pub secret_type: Option<String>,
    /// User-friendly name for the detected secret, matching the `secret_type`. For a list of built-in patterns, see \"[Supported secret scanning patterns](https://docs.github.com/code-security/secret-scanning/introduction/supported-secret-scanning-patterns#supported-secrets).\"
    #[serde(rename = "secret_type_display_name", skip_serializing_if = "Option::is_none")]
    pub secret_type_display_name: Option<String>,
    /// The token status as of the latest validity check.
    #[serde(rename = "validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<Validity>,
    /// Whether push protection was bypassed for the detected secret.
    #[serde(rename = "push_protection_bypassed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub push_protection_bypassed: Option<Option<bool>>,
    #[serde(rename = "push_protection_bypassed_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub push_protection_bypassed_by: Option<Option<Box<models::NullableSimpleUser>>>,
    /// The time that push protection was bypassed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "push_protection_bypassed_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub push_protection_bypassed_at: Option<Option<String>>,
}

impl SecretScanningAlertWebhook {
    pub fn new() -> SecretScanningAlertWebhook {
        SecretScanningAlertWebhook {
            number: None,
            created_at: None,
            updated_at: None,
            url: None,
            html_url: None,
            locations_url: None,
            resolution: None,
            resolved_at: None,
            resolved_by: None,
            resolution_comment: None,
            secret_type: None,
            secret_type_display_name: None,
            validity: None,
            push_protection_bypassed: None,
            push_protection_bypassed_by: None,
            push_protection_bypassed_at: None,
        }
    }
}
/// The token status as of the latest validity check.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Validity {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for Validity {
    fn default() -> Validity {
        Self::Active
    }
}

