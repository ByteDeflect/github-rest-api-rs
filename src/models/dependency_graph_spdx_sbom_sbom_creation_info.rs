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
pub struct DependencyGraphSpdxSbomSbomCreationInfo {
    /// The date and time the SPDX document was created.
    #[serde(rename = "created")]
    pub created: String,
    /// The tools that were used to generate the SPDX document.
    #[serde(rename = "creators")]
    pub creators: Vec<String>,
}

impl DependencyGraphSpdxSbomSbomCreationInfo {
    pub fn new(created: String, creators: Vec<String>) -> DependencyGraphSpdxSbomSbomCreationInfo {
        DependencyGraphSpdxSbomSbomCreationInfo {
            created,
            creators,
        }
    }
}

