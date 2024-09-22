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

/// DeploymentProtectionRule : Deployment protection rule
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentProtectionRule {
    /// The unique identifier for the deployment protection rule.
    #[serde(rename = "id")]
    pub id: i32,
    /// The node ID for the deployment protection rule.
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// Whether the deployment protection rule is enabled for the environment.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "app")]
    pub app: Box<models::CustomDeploymentRuleApp>,
}

impl DeploymentProtectionRule {
    /// Deployment protection rule
    pub fn new(id: i32, node_id: String, enabled: bool, app: models::CustomDeploymentRuleApp) -> DeploymentProtectionRule {
        DeploymentProtectionRule {
            id,
            node_id,
            enabled,
            app: Box::new(app),
        }
    }
}

