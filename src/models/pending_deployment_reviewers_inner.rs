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
pub struct PendingDeploymentReviewersInner {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::DeploymentReviewerType>,
    #[serde(rename = "reviewer", skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<Box<models::PendingDeploymentReviewersInnerReviewer>>,
}

impl PendingDeploymentReviewersInner {
    pub fn new() -> PendingDeploymentReviewersInner {
        PendingDeploymentReviewersInner {
            r#type: None,
            reviewer: None,
        }
    }
}

