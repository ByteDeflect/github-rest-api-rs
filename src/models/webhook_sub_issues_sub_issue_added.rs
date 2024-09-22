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
pub struct WebhookSubIssuesSubIssueAdded {
    #[serde(rename = "action")]
    pub action: Action,
    /// The ID of the sub-issue.
    #[serde(rename = "sub_issue_id")]
    pub sub_issue_id: f64,
    #[serde(rename = "sub_issue")]
    pub sub_issue: Box<models::Issue>,
    #[serde(rename = "sub_issue_repo")]
    pub sub_issue_repo: Box<models::Repository>,
    /// The ID of the parent issue.
    #[serde(rename = "parent_issue_id")]
    pub parent_issue_id: f64,
    #[serde(rename = "parent_issue")]
    pub parent_issue: Box<models::Issue>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::RepositoryWebhooks>>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<models::SimpleUserWebhooks>>,
}

impl WebhookSubIssuesSubIssueAdded {
    pub fn new(action: Action, sub_issue_id: f64, sub_issue: models::Issue, sub_issue_repo: models::Repository, parent_issue_id: f64, parent_issue: models::Issue) -> WebhookSubIssuesSubIssueAdded {
        WebhookSubIssuesSubIssueAdded {
            action,
            sub_issue_id,
            sub_issue: Box::new(sub_issue),
            sub_issue_repo: Box::new(sub_issue_repo),
            parent_issue_id,
            parent_issue: Box::new(parent_issue),
            installation: None,
            organization: None,
            repository: None,
            sender: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "sub_issue_added")]
    SubIssueAdded,
}

impl Default for Action {
    fn default() -> Action {
        Self::SubIssueAdded
    }
}

