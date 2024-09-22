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

/// RepositoryRulesetConditionsRepositoryNameTarget : Parameters for a repository name condition
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRulesetConditionsRepositoryNameTarget {
    #[serde(rename = "repository_name")]
    pub repository_name: Box<models::RepositoryRulesetConditionsRepositoryNameTargetRepositoryName>,
}

impl RepositoryRulesetConditionsRepositoryNameTarget {
    /// Parameters for a repository name condition
    pub fn new(repository_name: models::RepositoryRulesetConditionsRepositoryNameTargetRepositoryName) -> RepositoryRulesetConditionsRepositoryNameTarget {
        RepositoryRulesetConditionsRepositoryNameTarget {
            repository_name: Box::new(repository_name),
        }
    }
}

