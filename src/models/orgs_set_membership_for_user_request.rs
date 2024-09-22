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
pub struct OrgsSetMembershipForUserRequest {
    /// The role to give the user in the organization. Can be one of:    * `admin` - The user will become an owner of the organization.    * `member` - The user will become a non-owner member of the organization.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

impl OrgsSetMembershipForUserRequest {
    pub fn new() -> OrgsSetMembershipForUserRequest {
        OrgsSetMembershipForUserRequest {
            role: None,
        }
    }
}
/// The role to give the user in the organization. Can be one of:    * `admin` - The user will become an owner of the organization.    * `member` - The user will become a non-owner member of the organization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "member")]
    Member,
}

impl Default for Role {
    fn default() -> Role {
        Self::Admin
    }
}

