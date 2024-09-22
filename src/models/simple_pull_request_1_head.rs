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
pub struct SimplePullRequest1Head {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "repo", deserialize_with = "Option::deserialize")]
    pub repo: Option<Box<models::Repository9>>,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User1>>,
}

impl SimplePullRequest1Head {
    pub fn new(label: String, r#ref: String, repo: Option<models::Repository9>, sha: String, user: Option<models::User1>) -> SimplePullRequest1Head {
        SimplePullRequest1Head {
            label,
            r#ref,
            repo: if let Some(x) = repo {Some(Box::new(x))} else {None},
            sha,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
        }
    }
}

