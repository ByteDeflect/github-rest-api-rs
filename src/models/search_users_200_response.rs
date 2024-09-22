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
pub struct SearchUsers200Response {
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(rename = "incomplete_results")]
    pub incomplete_results: bool,
    #[serde(rename = "items")]
    pub items: Vec<models::UserSearchResultItem>,
}

impl SearchUsers200Response {
    pub fn new(total_count: i32, incomplete_results: bool, items: Vec<models::UserSearchResultItem>) -> SearchUsers200Response {
        SearchUsers200Response {
            total_count,
            incomplete_results,
            items,
        }
    }
}

