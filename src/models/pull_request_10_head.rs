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
pub struct PullRequest10Head {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "repo")]
    pub repo: Box<models::Repository13>,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User1>>,
}

impl PullRequest10Head {
    pub fn new(label: String, r#ref: String, repo: models::Repository13, sha: String, user: Option<models::User1>) -> PullRequest10Head {
        PullRequest10Head {
            label,
            r#ref,
            repo: Box::new(repo),
            sha,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
        }
    }
}

