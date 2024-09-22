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
pub struct WebhookCreate {
    /// The repository's current description.
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    /// The name of the repository's default branch (usually `main`).
    #[serde(rename = "master_branch")]
    pub master_branch: String,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    /// The pusher type for the event. Can be either `user` or a deploy key.
    #[serde(rename = "pusher_type")]
    pub pusher_type: String,
    /// The [`git ref`](https://docs.github.com/rest/git/refs#get-a-reference) resource.
    #[serde(rename = "ref")]
    pub r#ref: String,
    /// The type of Git ref object created in the repository.
    #[serde(rename = "ref_type")]
    pub ref_type: RefType,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookCreate {
    pub fn new(description: Option<String>, master_branch: String, pusher_type: String, r#ref: String, ref_type: RefType, repository: models::RepositoryWebhooks, sender: models::SimpleUserWebhooks) -> WebhookCreate {
        WebhookCreate {
            description,
            enterprise: None,
            installation: None,
            master_branch,
            organization: None,
            pusher_type,
            r#ref,
            ref_type,
            repository: Box::new(repository),
            sender: Box::new(sender),
        }
    }
}
/// The type of Git ref object created in the repository.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RefType {
    #[serde(rename = "tag")]
    Tag,
    #[serde(rename = "branch")]
    Branch,
}

impl Default for RefType {
    fn default() -> RefType {
        Self::Tag
    }
}

