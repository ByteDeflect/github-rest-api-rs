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
pub struct DependabotListRepoSecrets200Response {
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(rename = "secrets")]
    pub secrets: Vec<models::DependabotSecret>,
}

impl DependabotListRepoSecrets200Response {
    pub fn new(total_count: i32, secrets: Vec<models::DependabotSecret>) -> DependabotListRepoSecrets200Response {
        DependabotListRepoSecrets200Response {
            total_count,
            secrets,
        }
    }
}

