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
pub struct BranchWithProtectionLinks {
    #[serde(rename = "html")]
    pub html: String,
    #[serde(rename = "self")]
    pub param_self: String,
}

impl BranchWithProtectionLinks {
    pub fn new(html: String, param_self: String) -> BranchWithProtectionLinks {
        BranchWithProtectionLinks {
            html,
            param_self,
        }
    }
}

