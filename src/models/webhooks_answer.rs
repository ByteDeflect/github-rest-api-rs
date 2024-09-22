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
pub struct WebhooksAnswer {
    /// How the author is associated with the repository.
    #[serde(rename = "author_association")]
    pub author_association: AuthorAssociation,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "child_comment_count")]
    pub child_comment_count: i32,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "discussion_id")]
    pub discussion_id: i32,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "parent_id", deserialize_with = "Option::deserialize")]
    pub parent_id: Option<serde_json::Value>,
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Box<models::Reactions>>,
    #[serde(rename = "repository_url")]
    pub repository_url: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User1>>,
}

impl WebhooksAnswer {
    pub fn new(author_association: AuthorAssociation, body: String, child_comment_count: i32, created_at: String, discussion_id: i32, html_url: String, id: i32, node_id: String, parent_id: Option<serde_json::Value>, repository_url: String, updated_at: String, user: Option<models::User1>) -> WebhooksAnswer {
        WebhooksAnswer {
            author_association,
            body,
            child_comment_count,
            created_at,
            discussion_id,
            html_url,
            id,
            node_id,
            parent_id,
            reactions: None,
            repository_url,
            updated_at,
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

