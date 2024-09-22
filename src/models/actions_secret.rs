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

/// ActionsSecret : Set secrets for GitHub Actions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionsSecret {
    /// The name of the secret.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ActionsSecret {
    /// Set secrets for GitHub Actions.
    pub fn new(name: String, created_at: String, updated_at: String) -> ActionsSecret {
        ActionsSecret {
            name,
            created_at,
            updated_at,
        }
    }
}

