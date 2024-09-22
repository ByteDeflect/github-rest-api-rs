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

/// CheckRun : A check performed on the code of a given code change
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckRun {
    /// The id of the check.
    #[serde(rename = "id")]
    pub id: i64,
    /// The SHA of the commit that is being checked.
    #[serde(rename = "head_sha")]
    pub head_sha: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "external_id", deserialize_with = "Option::deserialize")]
    pub external_id: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "html_url", deserialize_with = "Option::deserialize")]
    pub html_url: Option<String>,
    #[serde(rename = "details_url", deserialize_with = "Option::deserialize")]
    pub details_url: Option<String>,
    /// The phase of the lifecycle that the check is currently in. Statuses of waiting, requested, and pending are reserved for GitHub Actions check runs.
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "conclusion", deserialize_with = "Option::deserialize")]
    pub conclusion: Option<Conclusion>,
    #[serde(rename = "started_at", deserialize_with = "Option::deserialize")]
    pub started_at: Option<String>,
    #[serde(rename = "completed_at", deserialize_with = "Option::deserialize")]
    pub completed_at: Option<String>,
    #[serde(rename = "output")]
    pub output: Box<models::CheckRunOutput>,
    /// The name of the check.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "check_suite", deserialize_with = "Option::deserialize")]
    pub check_suite: Option<Box<models::CheckRunCheckSuite>>,
    #[serde(rename = "app", deserialize_with = "Option::deserialize")]
    pub app: Option<Box<models::NullableIntegration>>,
    /// Pull requests that are open with a `head_sha` or `head_branch` that matches the check. The returned pull requests do not necessarily indicate pull requests that triggered the check.
    #[serde(rename = "pull_requests")]
    pub pull_requests: Vec<models::PullRequestMinimal>,
    #[serde(rename = "deployment", skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Box<models::DeploymentSimple>>,
}

impl CheckRun {
    /// A check performed on the code of a given code change
    pub fn new(id: i64, head_sha: String, node_id: String, external_id: Option<String>, url: String, html_url: Option<String>, details_url: Option<String>, status: Status, conclusion: Option<Conclusion>, started_at: Option<String>, completed_at: Option<String>, output: models::CheckRunOutput, name: String, check_suite: Option<models::CheckRunCheckSuite>, app: Option<models::NullableIntegration>, pull_requests: Vec<models::PullRequestMinimal>) -> CheckRun {
        CheckRun {
            id,
            head_sha,
            node_id,
            external_id,
            url,
            html_url,
            details_url,
            status,
            conclusion,
            started_at,
            completed_at,
            output: Box::new(output),
            name,
            check_suite: if let Some(x) = check_suite {Some(Box::new(x))} else {None},
            app: if let Some(x) = app {Some(Box::new(x))} else {None},
            pull_requests,
            deployment: None,
        }
    }
}
/// The phase of the lifecycle that the check is currently in. Statuses of waiting, requested, and pending are reserved for GitHub Actions check runs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "pending")]
    Pending,
}

impl Default for Status {
    fn default() -> Status {
        Self::Queued
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Conclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
}

impl Default for Conclusion {
    fn default() -> Conclusion {
        Self::Success
    }
}

