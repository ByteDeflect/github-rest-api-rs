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
pub struct SearchResultTextMatchesInner {
    #[serde(rename = "object_url", skip_serializing_if = "Option::is_none")]
    pub object_url: Option<String>,
    #[serde(rename = "object_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<Option<String>>,
    #[serde(rename = "property", skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    #[serde(rename = "fragment", skip_serializing_if = "Option::is_none")]
    pub fragment: Option<String>,
    #[serde(rename = "matches", skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<models::SearchResultTextMatchesInnerMatchesInner>>,
}

impl SearchResultTextMatchesInner {
    pub fn new() -> SearchResultTextMatchesInner {
        SearchResultTextMatchesInner {
            object_url: None,
            object_type: None,
            property: None,
            fragment: None,
            matches: None,
        }
    }
}

