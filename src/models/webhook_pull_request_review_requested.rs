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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookPullRequestReviewRequested {
    WebhookPullRequestReviewRequestedOneOf(Box<models::WebhookPullRequestReviewRequestedOneOf>),
    WebhookPullRequestReviewRequestedOneOf1(Box<models::WebhookPullRequestReviewRequestedOneOf1>),
}

impl Default for WebhookPullRequestReviewRequested {
    fn default() -> Self {
        Self::WebhookPullRequestReviewRequestedOneOf(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "review_requested")]
    ReviewRequested,
}

impl Default for Action {
    fn default() -> Action {
        Self::ReviewRequested
    }
}

