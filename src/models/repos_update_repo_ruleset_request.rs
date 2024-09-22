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
pub struct ReposUpdateRepoRulesetRequest {
    /// The name of the ruleset.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The target of the ruleset
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
    #[serde(rename = "enforcement", skip_serializing_if = "Option::is_none")]
    pub enforcement: Option<models::RepositoryRuleEnforcement>,
    /// The actors that can bypass the rules in this ruleset
    #[serde(rename = "bypass_actors", skip_serializing_if = "Option::is_none")]
    pub bypass_actors: Option<Vec<models::RepositoryRulesetBypassActor>>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Box<models::RepositoryRulesetConditions>>,
    /// An array of rules within the ruleset.
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<models::RepositoryRule>>,
}

impl ReposUpdateRepoRulesetRequest {
    pub fn new() -> ReposUpdateRepoRulesetRequest {
        ReposUpdateRepoRulesetRequest {
            name: None,
            target: None,
            enforcement: None,
            bypass_actors: None,
            conditions: None,
            rules: None,
        }
    }
}
/// The target of the ruleset
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Target {
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "tag")]
    Tag,
    #[serde(rename = "push")]
    Push,
}

impl Default for Target {
    fn default() -> Target {
        Self::Branch
    }
}

