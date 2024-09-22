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

/// CodeScanningAlertClassification : A classification of the file. For example to identify it as generated.
/// A classification of the file. For example to identify it as generated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CodeScanningAlertClassification {
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "generated")]
    Generated,
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "library")]
    Library,

}

impl std::fmt::Display for CodeScanningAlertClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Source => write!(f, "source"),
            Self::Generated => write!(f, "generated"),
            Self::Test => write!(f, "test"),
            Self::Library => write!(f, "library"),
        }
    }
}

impl Default for CodeScanningAlertClassification {
    fn default() -> CodeScanningAlertClassification {
        Self::Source
    }
}

