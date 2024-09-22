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

/// WebhookMemberAddedChangesPermission : This field is included for legacy purposes; use the `role_name` field instead. The `maintain` role is mapped to `write` and the `triage` role is mapped to `read`. To determine the role assigned to the collaborator, use the `role_name` field instead, which will provide the full role name, including custom roles.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookMemberAddedChangesPermission {
    #[serde(rename = "to")]
    pub to: To,
}

impl WebhookMemberAddedChangesPermission {
    /// This field is included for legacy purposes; use the `role_name` field instead. The `maintain` role is mapped to `write` and the `triage` role is mapped to `read`. To determine the role assigned to the collaborator, use the `role_name` field instead, which will provide the full role name, including custom roles.
    pub fn new(to: To) -> WebhookMemberAddedChangesPermission {
        WebhookMemberAddedChangesPermission {
            to,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum To {
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "read")]
    Read,
}

impl Default for To {
    fn default() -> To {
        Self::Write
    }
}

