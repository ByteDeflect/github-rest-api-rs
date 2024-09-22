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
pub struct WebhookRegistryPackagePublishedRegistryPackagePackageVersionRelease {
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::WebhooksSponsorshipMaintainer>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "draft", skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "prerelease", skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(rename = "published_at", skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "tag_name", skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(rename = "target_commitish", skip_serializing_if = "Option::is_none")]
    pub target_commitish: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl WebhookRegistryPackagePublishedRegistryPackagePackageVersionRelease {
    pub fn new() -> WebhookRegistryPackagePublishedRegistryPackagePackageVersionRelease {
        WebhookRegistryPackagePublishedRegistryPackagePackageVersionRelease {
            author: None,
            created_at: None,
            draft: None,
            html_url: None,
            id: None,
            name: None,
            prerelease: None,
            published_at: None,
            tag_name: None,
            target_commitish: None,
            url: None,
        }
    }
}

