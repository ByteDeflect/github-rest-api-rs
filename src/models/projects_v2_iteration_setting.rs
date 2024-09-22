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

/// ProjectsV2IterationSetting : An iteration setting for an iteration field
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectsV2IterationSetting {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "duration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Option<f64>>,
    #[serde(rename = "start_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
}

impl ProjectsV2IterationSetting {
    /// An iteration setting for an iteration field
    pub fn new(id: String, title: String) -> ProjectsV2IterationSetting {
        ProjectsV2IterationSetting {
            id,
            title,
            duration: None,
            start_date: None,
        }
    }
}

