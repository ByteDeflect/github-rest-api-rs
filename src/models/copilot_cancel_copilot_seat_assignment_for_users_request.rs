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
pub struct CopilotCancelCopilotSeatAssignmentForUsersRequest {
    /// The usernames of the organization members for which to revoke access to GitHub Copilot.
    #[serde(rename = "selected_usernames")]
    pub selected_usernames: Vec<String>,
}

impl CopilotCancelCopilotSeatAssignmentForUsersRequest {
    pub fn new(selected_usernames: Vec<String>) -> CopilotCancelCopilotSeatAssignmentForUsersRequest {
        CopilotCancelCopilotSeatAssignmentForUsersRequest {
            selected_usernames,
        }
    }
}

