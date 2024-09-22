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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReposSetUserAccessRestrictionsRequest {
    ReposSetUserAccessRestrictionsRequestOneOf(Box<models::ReposSetUserAccessRestrictionsRequestOneOf>),
    Array(Vec<String>),
}

impl Default for ReposSetUserAccessRestrictionsRequest {
    fn default() -> Self {
        Self::ReposSetUserAccessRestrictionsRequestOneOf(Default::default())
    }
}

