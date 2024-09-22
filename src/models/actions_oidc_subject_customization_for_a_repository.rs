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

/// ActionsOidcSubjectCustomizationForARepository : Actions OIDC subject customization for a repository
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionsOidcSubjectCustomizationForARepository {
    /// Whether to use the default template or not. If `true`, the `include_claim_keys` field is ignored.
    #[serde(rename = "use_default")]
    pub use_default: bool,
    /// Array of unique strings. Each claim key can only contain alphanumeric characters and underscores.
    #[serde(rename = "include_claim_keys", skip_serializing_if = "Option::is_none")]
    pub include_claim_keys: Option<Vec<String>>,
}

impl ActionsOidcSubjectCustomizationForARepository {
    /// Actions OIDC subject customization for a repository
    pub fn new(use_default: bool) -> ActionsOidcSubjectCustomizationForARepository {
        ActionsOidcSubjectCustomizationForARepository {
            use_default,
            include_claim_keys: None,
        }
    }
}

