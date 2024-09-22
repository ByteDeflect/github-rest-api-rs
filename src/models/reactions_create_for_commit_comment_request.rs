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
pub struct ReactionsCreateForCommitCommentRequest {
    /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the commit comment.
    #[serde(rename = "content")]
    pub content: Content,
}

impl ReactionsCreateForCommitCommentRequest {
    pub fn new(content: Content) -> ReactionsCreateForCommitCommentRequest {
        ReactionsCreateForCommitCommentRequest {
            content,
        }
    }
}
/// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the commit comment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Content {
    #[serde(rename = "+1")]
    Plus1,
    #[serde(rename = "-1")]
    Variant1,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
}

impl Default for Content {
    fn default() -> Content {
        Self::Plus1
    }
}

