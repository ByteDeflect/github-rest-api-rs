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
pub struct WorkflowStep4 {
    #[serde(rename = "completed_at", deserialize_with = "Option::deserialize")]
    pub completed_at: Option<String>,
    #[serde(rename = "conclusion", deserialize_with = "Option::deserialize")]
    pub conclusion: Option<Conclusion>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "started_at", deserialize_with = "Option::deserialize")]
    pub started_at: Option<String>,
    #[serde(rename = "status")]
    pub status: Status,
}

impl WorkflowStep4 {
    pub fn new(completed_at: Option<String>, conclusion: Option<Conclusion>, name: String, number: i32, started_at: Option<String>, status: Status) -> WorkflowStep4 {
        WorkflowStep4 {
            completed_at,
            conclusion,
            name,
            number,
            started_at,
            status,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Conclusion {
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "null")]
    Null,
}

impl Default for Conclusion {
    fn default() -> Conclusion {
        Self::Failure
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "waiting")]
    Waiting,
}

impl Default for Status {
    fn default() -> Status {
        Self::Completed
    }
}

