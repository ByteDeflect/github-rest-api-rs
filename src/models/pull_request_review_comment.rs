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

/// PullRequestReviewComment : The [comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request) itself.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequestReviewComment {
    #[serde(rename = "_links")]
    pub _links: Box<models::WebhooksReviewCommentLinks>,
    /// How the author is associated with the repository.
    #[serde(rename = "author_association")]
    pub author_association: AuthorAssociation,
    /// The text of the comment.
    #[serde(rename = "body")]
    pub body: String,
    /// The SHA of the commit to which the comment applies.
    #[serde(rename = "commit_id")]
    pub commit_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The diff of the line that the comment refers to.
    #[serde(rename = "diff_hunk")]
    pub diff_hunk: String,
    /// HTML URL for the pull request review comment.
    #[serde(rename = "html_url")]
    pub html_url: String,
    /// The ID of the pull request review comment.
    #[serde(rename = "id")]
    pub id: i32,
    /// The comment ID to reply to.
    #[serde(rename = "in_reply_to_id", skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<i32>,
    /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(rename = "line", deserialize_with = "Option::deserialize")]
    pub line: Option<i32>,
    /// The node ID of the pull request review comment.
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// The SHA of the original commit to which the comment applies.
    #[serde(rename = "original_commit_id")]
    pub original_commit_id: String,
    /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(rename = "original_line", deserialize_with = "Option::deserialize")]
    pub original_line: Option<i32>,
    /// The index of the original line in the diff to which the comment applies.
    #[serde(rename = "original_position")]
    pub original_position: i32,
    /// The first line of the range for a multi-line comment.
    #[serde(rename = "original_start_line", deserialize_with = "Option::deserialize")]
    pub original_start_line: Option<i32>,
    /// The relative path of the file to which the comment applies.
    #[serde(rename = "path")]
    pub path: String,
    /// The line index in the diff to which the comment applies.
    #[serde(rename = "position", deserialize_with = "Option::deserialize")]
    pub position: Option<i32>,
    /// The ID of the pull request review to which the comment belongs.
    #[serde(rename = "pull_request_review_id", deserialize_with = "Option::deserialize")]
    pub pull_request_review_id: Option<i32>,
    /// URL for the pull request that the review comment belongs to.
    #[serde(rename = "pull_request_url")]
    pub pull_request_url: String,
    #[serde(rename = "reactions")]
    pub reactions: Box<models::Reactions>,
    /// The side of the first line of the range for a multi-line comment.
    #[serde(rename = "side")]
    pub side: Side,
    /// The first line of the range for a multi-line comment.
    #[serde(rename = "start_line", deserialize_with = "Option::deserialize")]
    pub start_line: Option<i32>,
    /// The side of the first line of the range for a multi-line comment.
    #[serde(rename = "start_side", deserialize_with = "Option::deserialize")]
    pub start_side: Option<StartSide>,
    /// The level at which the comment is targeted, can be a diff line or a file.
    #[serde(rename = "subject_type", skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<SubjectType>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// URL for the pull request review comment
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User1>>,
}

impl PullRequestReviewComment {
    /// The [comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request) itself.
    pub fn new(_links: models::WebhooksReviewCommentLinks, author_association: AuthorAssociation, body: String, commit_id: String, created_at: String, diff_hunk: String, html_url: String, id: i32, line: Option<i32>, node_id: String, original_commit_id: String, original_line: Option<i32>, original_position: i32, original_start_line: Option<i32>, path: String, position: Option<i32>, pull_request_review_id: Option<i32>, pull_request_url: String, reactions: models::Reactions, side: Side, start_line: Option<i32>, start_side: Option<StartSide>, updated_at: String, url: String, user: Option<models::User1>) -> PullRequestReviewComment {
        PullRequestReviewComment {
            _links: Box::new(_links),
            author_association,
            body,
            commit_id,
            created_at,
            diff_hunk,
            html_url,
            id,
            in_reply_to_id: None,
            line,
            node_id,
            original_commit_id,
            original_line,
            original_position,
            original_start_line,
            path,
            position,
            pull_request_review_id,
            pull_request_url,
            reactions: Box::new(reactions),
            side,
            start_line,
            start_side,
            subject_type: None,
            updated_at,
            url,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
        }
    }
}
/// How the author is associated with the repository.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthorAssociation {
    #[serde(rename = "COLLABORATOR")]
    Collaborator,
    #[serde(rename = "CONTRIBUTOR")]
    Contributor,
    #[serde(rename = "FIRST_TIMER")]
    FirstTimer,
    #[serde(rename = "FIRST_TIME_CONTRIBUTOR")]
    FirstTimeContributor,
    #[serde(rename = "MANNEQUIN")]
    Mannequin,
    #[serde(rename = "MEMBER")]
    Member,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "OWNER")]
    Owner,
}

impl Default for AuthorAssociation {
    fn default() -> AuthorAssociation {
        Self::Collaborator
    }
}
/// The side of the first line of the range for a multi-line comment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}

impl Default for Side {
    fn default() -> Side {
        Self::Left
    }
}
/// The side of the first line of the range for a multi-line comment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StartSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
    #[serde(rename = "null")]
    Null,
}

impl Default for StartSide {
    fn default() -> StartSide {
        Self::Left
    }
}
/// The level at which the comment is targeted, can be a diff line or a file.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubjectType {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "file")]
    File,
}

impl Default for SubjectType {
    fn default() -> SubjectType {
        Self::Line
    }
}

