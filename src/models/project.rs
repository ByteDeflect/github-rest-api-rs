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

/// Project : Projects are a way to organize columns and cards of work.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "owner_url")]
    pub owner_url: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "columns_url")]
    pub columns_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// Name of the project
    #[serde(rename = "name")]
    pub name: String,
    /// Body of the project
    #[serde(rename = "body", deserialize_with = "Option::deserialize")]
    pub body: Option<String>,
    #[serde(rename = "number")]
    pub number: i32,
    /// State of the project; either 'open' or 'closed'
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "creator", deserialize_with = "Option::deserialize")]
    pub creator: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The baseline permission that all organization members have on this project. Only present if owner is an organization.
    #[serde(rename = "organization_permission", skip_serializing_if = "Option::is_none")]
    pub organization_permission: Option<OrganizationPermission>,
    /// Whether or not this project can be seen by everyone. Only present if owner is an organization.
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
}

impl Project {
    /// Projects are a way to organize columns and cards of work.
    pub fn new(owner_url: String, url: String, html_url: String, columns_url: String, id: i32, node_id: String, name: String, body: Option<String>, number: i32, state: String, creator: Option<models::NullableSimpleUser>, created_at: String, updated_at: String) -> Project {
        Project {
            owner_url,
            url,
            html_url,
            columns_url,
            id,
            node_id,
            name,
            body,
            number,
            state,
            creator: if let Some(x) = creator {Some(Box::new(x))} else {None},
            created_at,
            updated_at,
            organization_permission: None,
            private: None,
        }
    }
}
/// The baseline permission that all organization members have on this project. Only present if owner is an organization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrganizationPermission {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "none")]
    None,
}

impl Default for OrganizationPermission {
    fn default() -> OrganizationPermission {
        Self::Read
    }
}

