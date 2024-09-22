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

/// CodeScanningVariantAnalysis : A run of a CodeQL query against one or more repositories.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningVariantAnalysis {
    /// The ID of the variant analysis.
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "controller_repo")]
    pub controller_repo: Box<models::SimpleRepository>,
    #[serde(rename = "actor")]
    pub actor: Box<models::SimpleUser>,
    #[serde(rename = "query_language")]
    pub query_language: models::CodeScanningVariantAnalysisLanguage,
    /// The download url for the query pack.
    #[serde(rename = "query_pack_url")]
    pub query_pack_url: String,
    /// The date and time at which the variant analysis was created, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The date and time at which the variant analysis was last updated, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The date and time at which the variant analysis was completed, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ. Will be null if the variant analysis has not yet completed or this information is not available.
    #[serde(rename = "completed_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<Option<String>>,
    #[serde(rename = "status")]
    pub status: Status,
    /// The GitHub Actions workflow run used to execute this variant analysis. This is only available if the workflow run has started.
    #[serde(rename = "actions_workflow_run_id", skip_serializing_if = "Option::is_none")]
    pub actions_workflow_run_id: Option<i32>,
    /// The reason for a failure of the variant analysis. This is only available if the variant analysis has failed.
    #[serde(rename = "failure_reason", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<FailureReason>,
    #[serde(rename = "scanned_repositories", skip_serializing_if = "Option::is_none")]
    pub scanned_repositories: Option<Vec<models::CodeScanningVariantAnalysisScannedRepositoriesInner>>,
    #[serde(rename = "skipped_repositories", skip_serializing_if = "Option::is_none")]
    pub skipped_repositories: Option<Box<models::CodeScanningVariantAnalysisSkippedRepositories>>,
}

impl CodeScanningVariantAnalysis {
    /// A run of a CodeQL query against one or more repositories.
    pub fn new(id: i32, controller_repo: models::SimpleRepository, actor: models::SimpleUser, query_language: models::CodeScanningVariantAnalysisLanguage, query_pack_url: String, status: Status) -> CodeScanningVariantAnalysis {
        CodeScanningVariantAnalysis {
            id,
            controller_repo: Box::new(controller_repo),
            actor: Box::new(actor),
            query_language,
            query_pack_url,
            created_at: None,
            updated_at: None,
            completed_at: None,
            status,
            actions_workflow_run_id: None,
            failure_reason: None,
            scanned_repositories: None,
            skipped_repositories: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl Default for Status {
    fn default() -> Status {
        Self::InProgress
    }
}
/// The reason for a failure of the variant analysis. This is only available if the variant analysis has failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FailureReason {
    #[serde(rename = "no_repos_queried")]
    NoReposQueried,
    #[serde(rename = "actions_workflow_run_failed")]
    ActionsWorkflowRunFailed,
    #[serde(rename = "internal_error")]
    InternalError,
}

impl Default for FailureReason {
    fn default() -> FailureReason {
        Self::NoReposQueried
    }
}

