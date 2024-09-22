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
pub struct ProjectsCreateCardRequestOneOf1 {
    /// The unique identifier of the content associated with the card
    #[serde(rename = "content_id")]
    pub content_id: i32,
    /// The piece of content associated with the card
    #[serde(rename = "content_type")]
    pub content_type: String,
}

impl ProjectsCreateCardRequestOneOf1 {
    pub fn new(content_id: i32, content_type: String) -> ProjectsCreateCardRequestOneOf1 {
        ProjectsCreateCardRequestOneOf1 {
            content_id,
            content_type,
        }
    }
}

