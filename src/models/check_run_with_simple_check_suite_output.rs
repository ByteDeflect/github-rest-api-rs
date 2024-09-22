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
pub struct CheckRunWithSimpleCheckSuiteOutput {
    #[serde(rename = "annotations_count")]
    pub annotations_count: i32,
    #[serde(rename = "annotations_url")]
    pub annotations_url: String,
    #[serde(rename = "summary", deserialize_with = "Option::deserialize")]
    pub summary: Option<String>,
    #[serde(rename = "text", deserialize_with = "Option::deserialize")]
    pub text: Option<String>,
    #[serde(rename = "title", deserialize_with = "Option::deserialize")]
    pub title: Option<String>,
}

impl CheckRunWithSimpleCheckSuiteOutput {
    pub fn new(annotations_count: i32, annotations_url: String, summary: Option<String>, text: Option<String>, title: Option<String>) -> CheckRunWithSimpleCheckSuiteOutput {
        CheckRunWithSimpleCheckSuiteOutput {
            annotations_count,
            annotations_url,
            summary,
            text,
            title,
        }
    }
}

