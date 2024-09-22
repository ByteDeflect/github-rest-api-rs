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

/// CustomDeploymentRuleApp : A GitHub App that is providing a custom deployment protection rule.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDeploymentRuleApp {
    /// The unique identifier of the deployment protection rule integration.
    #[serde(rename = "id")]
    pub id: i32,
    /// The slugified name of the deployment protection rule integration.
    #[serde(rename = "slug")]
    pub slug: String,
    /// The URL for the endpoint to get details about the app.
    #[serde(rename = "integration_url")]
    pub integration_url: String,
    /// The node ID for the deployment protection rule integration.
    #[serde(rename = "node_id")]
    pub node_id: String,
}

impl CustomDeploymentRuleApp {
    /// A GitHub App that is providing a custom deployment protection rule.
    pub fn new(id: i32, slug: String, integration_url: String, node_id: String) -> CustomDeploymentRuleApp {
        CustomDeploymentRuleApp {
            id,
            slug,
            integration_url,
            node_id,
        }
    }
}

