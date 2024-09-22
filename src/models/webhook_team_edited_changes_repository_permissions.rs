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
pub struct WebhookTeamEditedChangesRepositoryPermissions {
    #[serde(rename = "from")]
    pub from: Box<models::WebhookTeamEditedChangesRepositoryPermissionsFrom>,
}

impl WebhookTeamEditedChangesRepositoryPermissions {
    pub fn new(from: models::WebhookTeamEditedChangesRepositoryPermissionsFrom) -> WebhookTeamEditedChangesRepositoryPermissions {
        WebhookTeamEditedChangesRepositoryPermissions {
            from: Box::new(from),
        }
    }
}

