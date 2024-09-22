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
pub struct OrgsCreateOrUpdateCustomPropertiesRequest {
    /// The array of custom properties to create or update.
    #[serde(rename = "properties")]
    pub properties: Vec<models::CustomProperty>,
}

impl OrgsCreateOrUpdateCustomPropertiesRequest {
    pub fn new(properties: Vec<models::CustomProperty>) -> OrgsCreateOrUpdateCustomPropertiesRequest {
        OrgsCreateOrUpdateCustomPropertiesRequest {
            properties,
        }
    }
}

