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
pub struct WorkflowRunUsageBillable {
    #[serde(rename = "UBUNTU", skip_serializing_if = "Option::is_none")]
    pub ubuntu: Option<Box<models::WorkflowRunUsageBillableUbuntu>>,
    #[serde(rename = "MACOS", skip_serializing_if = "Option::is_none")]
    pub macos: Option<Box<models::WorkflowRunUsageBillableUbuntu>>,
    #[serde(rename = "WINDOWS", skip_serializing_if = "Option::is_none")]
    pub windows: Option<Box<models::WorkflowRunUsageBillableUbuntu>>,
}

impl WorkflowRunUsageBillable {
    pub fn new() -> WorkflowRunUsageBillable {
        WorkflowRunUsageBillable {
            ubuntu: None,
            macos: None,
            windows: None,
        }
    }
}

