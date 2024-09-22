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

/// ActionsCacheList : Repository actions caches
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionsCacheList {
    /// Total number of caches
    #[serde(rename = "total_count")]
    pub total_count: i32,
    /// Array of caches
    #[serde(rename = "actions_caches")]
    pub actions_caches: Vec<models::ActionsCacheListActionsCachesInner>,
}

impl ActionsCacheList {
    /// Repository actions caches
    pub fn new(total_count: i32, actions_caches: Vec<models::ActionsCacheListActionsCachesInner>) -> ActionsCacheList {
        ActionsCacheList {
            total_count,
            actions_caches,
        }
    }
}

