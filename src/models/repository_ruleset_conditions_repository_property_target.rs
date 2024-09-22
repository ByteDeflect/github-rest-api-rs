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

/// RepositoryRulesetConditionsRepositoryPropertyTarget : Parameters for a repository property condition
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRulesetConditionsRepositoryPropertyTarget {
    #[serde(rename = "repository_property")]
    pub repository_property: Box<models::RepositoryRulesetConditionsRepositoryPropertyTargetRepositoryProperty>,
}

impl RepositoryRulesetConditionsRepositoryPropertyTarget {
    /// Parameters for a repository property condition
    pub fn new(repository_property: models::RepositoryRulesetConditionsRepositoryPropertyTargetRepositoryProperty) -> RepositoryRulesetConditionsRepositoryPropertyTarget {
        RepositoryRulesetConditionsRepositoryPropertyTarget {
            repository_property: Box::new(repository_property),
        }
    }
}

