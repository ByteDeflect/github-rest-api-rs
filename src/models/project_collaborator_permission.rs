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

/// ProjectCollaboratorPermission : Project Collaborator Permission
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectCollaboratorPermission {
    #[serde(rename = "permission")]
    pub permission: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::NullableSimpleUser>>,
}

impl ProjectCollaboratorPermission {
    /// Project Collaborator Permission
    pub fn new(permission: String, user: Option<models::NullableSimpleUser>) -> ProjectCollaboratorPermission {
        ProjectCollaboratorPermission {
            permission,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
        }
    }
}

