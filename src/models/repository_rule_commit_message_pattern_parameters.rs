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
pub struct RepositoryRuleCommitMessagePatternParameters {
    /// How this rule will appear to users.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If true, the rule will fail if the pattern matches.
    #[serde(rename = "negate", skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    /// The operator to use for matching.
    #[serde(rename = "operator")]
    pub operator: Operator,
    /// The pattern to match with.
    #[serde(rename = "pattern")]
    pub pattern: String,
}

impl RepositoryRuleCommitMessagePatternParameters {
    pub fn new(operator: Operator, pattern: String) -> RepositoryRuleCommitMessagePatternParameters {
        RepositoryRuleCommitMessagePatternParameters {
            name: None,
            negate: None,
            operator,
            pattern,
        }
    }
}
/// The operator to use for matching.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "starts_with")]
    StartsWith,
    #[serde(rename = "ends_with")]
    EndsWith,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "regex")]
    Regex,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::StartsWith
    }
}

