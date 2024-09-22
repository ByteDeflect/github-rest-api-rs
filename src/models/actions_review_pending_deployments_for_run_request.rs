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
pub struct ActionsReviewPendingDeploymentsForRunRequest {
    /// The list of environment ids to approve or reject
    #[serde(rename = "environment_ids")]
    pub environment_ids: Vec<i32>,
    /// Whether to approve or reject deployment to the specified environments.
    #[serde(rename = "state")]
    pub state: State,
    /// A comment to accompany the deployment review
    #[serde(rename = "comment")]
    pub comment: String,
}

impl ActionsReviewPendingDeploymentsForRunRequest {
    pub fn new(environment_ids: Vec<i32>, state: State, comment: String) -> ActionsReviewPendingDeploymentsForRunRequest {
        ActionsReviewPendingDeploymentsForRunRequest {
            environment_ids,
            state,
            comment,
        }
    }
}
/// Whether to approve or reject deployment to the specified environments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
}

impl Default for State {
    fn default() -> State {
        Self::Approved
    }
}

