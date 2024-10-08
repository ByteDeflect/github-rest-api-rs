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

/// CopilotSeatDetailsOrganization : The organization to which this seat belongs.
/// The organization to which this seat belongs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopilotSeatDetailsOrganization {
    OrganizationSimple(Box<models::OrganizationSimple>),
}

impl Default for CopilotSeatDetailsOrganization {
    fn default() -> Self {
        Self::OrganizationSimple(Default::default())
    }
}

