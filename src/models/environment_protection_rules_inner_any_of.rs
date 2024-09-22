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
pub struct EnvironmentProtectionRulesInnerAnyOf {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    /// The amount of time to delay a job after the job is initially triggered. The time (in minutes) must be an integer between 0 and 43,200 (30 days).
    #[serde(rename = "wait_timer", skip_serializing_if = "Option::is_none")]
    pub wait_timer: Option<i32>,
}

impl EnvironmentProtectionRulesInnerAnyOf {
    pub fn new(id: i32, node_id: String, r#type: String) -> EnvironmentProtectionRulesInnerAnyOf {
        EnvironmentProtectionRulesInnerAnyOf {
            id,
            node_id,
            r#type,
            wait_timer: None,
        }
    }
}

