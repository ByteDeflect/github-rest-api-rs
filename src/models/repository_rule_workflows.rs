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

/// RepositoryRuleWorkflows : Require all changes made to a targeted branch to pass the specified workflows before they can be merged.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRuleWorkflows {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<models::RepositoryRuleWorkflowsParameters>>,
}

impl RepositoryRuleWorkflows {
    /// Require all changes made to a targeted branch to pass the specified workflows before they can be merged.
    pub fn new(r#type: Type) -> RepositoryRuleWorkflows {
        RepositoryRuleWorkflows {
            r#type,
            parameters: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "workflows")]
    Workflows,
}

impl Default for Type {
    fn default() -> Type {
        Self::Workflows
    }
}

