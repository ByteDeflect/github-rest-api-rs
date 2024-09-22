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
pub struct TeamsUpdateInOrgRequest {
    /// The name of the team.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the team.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The level of privacy this team should have. Editing teams without specifying this parameter leaves `privacy` intact. When a team is nested, the `privacy` for parent teams cannot be `secret`. The options are:   **For a non-nested team:**    * `secret` - only visible to organization owners and members of this team.    * `closed` - visible to all members of this organization.   **For a parent or child team:**    * `closed` - visible to all members of this organization.
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
    /// The notification setting the team has chosen. Editing teams without specifying this parameter leaves `notification_setting` intact. The options are:   * `notifications_enabled` - team members receive notifications when the team is @mentioned.    * `notifications_disabled` - no one receives notifications.
    #[serde(rename = "notification_setting", skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<NotificationSetting>,
    /// **Deprecated**. The permission that new repositories will be added to the team with when none is specified.
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
    /// The ID of a team to set as the parent team.
    #[serde(rename = "parent_team_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_team_id: Option<Option<i32>>,
}

impl TeamsUpdateInOrgRequest {
    pub fn new() -> TeamsUpdateInOrgRequest {
        TeamsUpdateInOrgRequest {
            name: None,
            description: None,
            privacy: None,
            notification_setting: None,
            permission: None,
            parent_team_id: None,
        }
    }
}
/// The level of privacy this team should have. Editing teams without specifying this parameter leaves `privacy` intact. When a team is nested, the `privacy` for parent teams cannot be `secret`. The options are:   **For a non-nested team:**    * `secret` - only visible to organization owners and members of this team.    * `closed` - visible to all members of this organization.   **For a parent or child team:**    * `closed` - visible to all members of this organization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Privacy {
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "closed")]
    Closed,
}

impl Default for Privacy {
    fn default() -> Privacy {
        Self::Secret
    }
}
/// The notification setting the team has chosen. Editing teams without specifying this parameter leaves `notification_setting` intact. The options are:   * `notifications_enabled` - team members receive notifications when the team is @mentioned.    * `notifications_disabled` - no one receives notifications.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationSetting {
    #[serde(rename = "notifications_enabled")]
    Enabled,
    #[serde(rename = "notifications_disabled")]
    Disabled,
}

impl Default for NotificationSetting {
    fn default() -> NotificationSetting {
        Self::Enabled
    }
}
/// **Deprecated**. The permission that new repositories will be added to the team with when none is specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "pull")]
    Pull,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "admin")]
    Admin,
}

impl Default for Permission {
    fn default() -> Permission {
        Self::Pull
    }
}

