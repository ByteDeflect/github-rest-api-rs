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

/// ReposUpdateBranchProtectionRequestRequiredPullRequestReviews : Require at least one approving review on a pull request, before merging. Set to `null` to disable.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReposUpdateBranchProtectionRequestRequiredPullRequestReviews {
    #[serde(rename = "dismissal_restrictions", skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions: Option<Box<models::ReposUpdateBranchProtectionRequestRequiredPullRequestReviewsDismissalRestrictions>>,
    /// Set to `true` if you want to automatically dismiss approving reviews when someone pushes a new commit.
    #[serde(rename = "dismiss_stale_reviews", skip_serializing_if = "Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,
    /// Blocks merging pull requests until [code owners](https://docs.github.com/articles/about-code-owners/) review them.
    #[serde(rename = "require_code_owner_reviews", skip_serializing_if = "Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,
    /// Specify the number of reviewers required to approve pull requests. Use a number between 1 and 6 or 0 to not require reviewers.
    #[serde(rename = "required_approving_review_count", skip_serializing_if = "Option::is_none")]
    pub required_approving_review_count: Option<i32>,
    /// Whether the most recent push must be approved by someone other than the person who pushed it. Default: `false`.
    #[serde(rename = "require_last_push_approval", skip_serializing_if = "Option::is_none")]
    pub require_last_push_approval: Option<bool>,
    #[serde(rename = "bypass_pull_request_allowances", skip_serializing_if = "Option::is_none")]
    pub bypass_pull_request_allowances: Option<Box<models::ReposUpdateBranchProtectionRequestRequiredPullRequestReviewsBypassPullRequestAllowances>>,
}

impl ReposUpdateBranchProtectionRequestRequiredPullRequestReviews {
    /// Require at least one approving review on a pull request, before merging. Set to `null` to disable.
    pub fn new() -> ReposUpdateBranchProtectionRequestRequiredPullRequestReviews {
        ReposUpdateBranchProtectionRequestRequiredPullRequestReviews {
            dismissal_restrictions: None,
            dismiss_stale_reviews: None,
            require_code_owner_reviews: None,
            required_approving_review_count: None,
            require_last_push_approval: None,
            bypass_pull_request_allowances: None,
        }
    }
}

