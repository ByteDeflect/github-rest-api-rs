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
pub struct ApiOverviewDomainsArtifactAttestations {
    #[serde(rename = "trust_domain", skip_serializing_if = "Option::is_none")]
    pub trust_domain: Option<String>,
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

impl ApiOverviewDomainsArtifactAttestations {
    pub fn new() -> ApiOverviewDomainsArtifactAttestations {
        ApiOverviewDomainsArtifactAttestations {
            trust_domain: None,
            services: None,
        }
    }
}

