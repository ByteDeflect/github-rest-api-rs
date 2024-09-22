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
pub struct DependencyGraphDiffInnerVulnerabilitiesInner {
    #[serde(rename = "severity")]
    pub severity: String,
    #[serde(rename = "advisory_ghsa_id")]
    pub advisory_ghsa_id: String,
    #[serde(rename = "advisory_summary")]
    pub advisory_summary: String,
    #[serde(rename = "advisory_url")]
    pub advisory_url: String,
}

impl DependencyGraphDiffInnerVulnerabilitiesInner {
    pub fn new(severity: String, advisory_ghsa_id: String, advisory_summary: String, advisory_url: String) -> DependencyGraphDiffInnerVulnerabilitiesInner {
        DependencyGraphDiffInnerVulnerabilitiesInner {
            severity,
            advisory_ghsa_id,
            advisory_summary,
            advisory_url,
        }
    }
}

