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
pub struct ActionsListSelfHostedRunnersForOrg200Response {
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(rename = "runners")]
    pub runners: Vec<models::Runner>,
}

impl ActionsListSelfHostedRunnersForOrg200Response {
    pub fn new(total_count: i32, runners: Vec<models::Runner>) -> ActionsListSelfHostedRunnersForOrg200Response {
        ActionsListSelfHostedRunnersForOrg200Response {
            total_count,
            runners,
        }
    }
}

