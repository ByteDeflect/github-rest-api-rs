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

/// Actor : Actor
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "display_login", skip_serializing_if = "Option::is_none")]
    pub display_login: Option<String>,
    #[serde(rename = "gravatar_id", deserialize_with = "Option::deserialize")]
    pub gravatar_id: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
}

impl Actor {
    /// Actor
    pub fn new(id: i32, login: String, gravatar_id: Option<String>, url: String, avatar_url: String) -> Actor {
        Actor {
            id,
            login,
            display_login: None,
            gravatar_id,
            url,
            avatar_url,
        }
    }
}

