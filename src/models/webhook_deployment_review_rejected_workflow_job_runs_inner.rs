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
pub struct WebhookDeploymentReviewRejectedWorkflowJobRunsInner {
    #[serde(rename = "conclusion", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<Option<String>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl WebhookDeploymentReviewRejectedWorkflowJobRunsInner {
    pub fn new() -> WebhookDeploymentReviewRejectedWorkflowJobRunsInner {
        WebhookDeploymentReviewRejectedWorkflowJobRunsInner {
            conclusion: None,
            created_at: None,
            environment: None,
            html_url: None,
            id: None,
            name: None,
            status: None,
            updated_at: None,
        }
    }
}

