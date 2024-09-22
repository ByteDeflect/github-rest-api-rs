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

/// OrganizationRole : Organization roles
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationRole {
    /// The unique identifier of the role.
    #[serde(rename = "id")]
    pub id: i64,
    /// The name of the role.
    #[serde(rename = "name")]
    pub name: String,
    /// A short description about who this role is for or what permissions it grants.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// A list of permissions included in this role.
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
    #[serde(rename = "organization", deserialize_with = "Option::deserialize")]
    pub organization: Option<Box<models::NullableSimpleUser>>,
    /// The date and time the role was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date and time the role was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl OrganizationRole {
    /// Organization roles
    pub fn new(id: i64, name: String, permissions: Vec<String>, organization: Option<models::NullableSimpleUser>, created_at: String, updated_at: String) -> OrganizationRole {
        OrganizationRole {
            id,
            name,
            description: None,
            permissions,
            organization: if let Some(x) = organization {Some(Box::new(x))} else {None},
            created_at,
            updated_at,
        }
    }
}

