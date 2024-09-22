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
pub struct ApiOverviewSshKeyFingerprints {
    #[serde(rename = "SHA256_RSA", skip_serializing_if = "Option::is_none")]
    pub sha256_rsa: Option<String>,
    #[serde(rename = "SHA256_DSA", skip_serializing_if = "Option::is_none")]
    pub sha256_dsa: Option<String>,
    #[serde(rename = "SHA256_ECDSA", skip_serializing_if = "Option::is_none")]
    pub sha256_ecdsa: Option<String>,
    #[serde(rename = "SHA256_ED25519", skip_serializing_if = "Option::is_none")]
    pub sha256_ed25519: Option<String>,
}

impl ApiOverviewSshKeyFingerprints {
    pub fn new() -> ApiOverviewSshKeyFingerprints {
        ApiOverviewSshKeyFingerprints {
            sha256_rsa: None,
            sha256_dsa: None,
            sha256_ecdsa: None,
            sha256_ed25519: None,
        }
    }
}

