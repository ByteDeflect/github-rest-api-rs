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

/// SecretScanningLocationPullRequestReviewComment : Represents a 'pull_request_review_comment' secret scanning location type. This location type shows that a secret was detected in a review comment on a pull request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretScanningLocationPullRequestReviewComment {
    /// The API URL to get the pull request review comment where the secret was detected.
    #[serde(rename = "pull_request_review_comment_url")]
    pub pull_request_review_comment_url: String,
}

impl SecretScanningLocationPullRequestReviewComment {
    /// Represents a 'pull_request_review_comment' secret scanning location type. This location type shows that a secret was detected in a review comment on a pull request.
    pub fn new(pull_request_review_comment_url: String) -> SecretScanningLocationPullRequestReviewComment {
        SecretScanningLocationPullRequestReviewComment {
            pull_request_review_comment_url,
        }
    }
}

