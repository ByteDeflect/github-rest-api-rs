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
pub struct ReposSetTeamAccessRestrictionsRequestOneOf {
    /// The slug values for teams
    #[serde(rename = "teams")]
    pub teams: Vec<String>,
}

impl ReposSetTeamAccessRestrictionsRequestOneOf {
    pub fn new(teams: Vec<String>) -> ReposSetTeamAccessRestrictionsRequestOneOf {
        ReposSetTeamAccessRestrictionsRequestOneOf {
            teams,
        }
    }
}

