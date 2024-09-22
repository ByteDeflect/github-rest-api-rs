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
pub struct WebhookRegistryPackageUpdatedRegistryPackagePackageVersion {
    #[serde(rename = "author")]
    pub author: Box<models::WebhookRegistryPackagePublishedRegistryPackageOwner>,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "body_html")]
    pub body_html: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "docker_metadata", skip_serializing_if = "Option::is_none")]
    pub docker_metadata: Option<Vec<models::WebhookRegistryPackageUpdatedRegistryPackagePackageVersionDockerMetadataInner>>,
    #[serde(rename = "draft", skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "installation_command")]
    pub installation_command: String,
    #[serde(rename = "manifest", skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: Vec<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "package_files")]
    pub package_files: Vec<models::WebhookRegistryPackageUpdatedRegistryPackagePackageVersionPackageFilesInner>,
    #[serde(rename = "package_url")]
    pub package_url: String,
    #[serde(rename = "prerelease", skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(rename = "release", skip_serializing_if = "Option::is_none")]
    pub release: Option<Box<models::WebhookRegistryPackageUpdatedRegistryPackagePackageVersionRelease>>,
    #[serde(rename = "rubygems_metadata", skip_serializing_if = "Option::is_none")]
    pub rubygems_metadata: Option<Vec<models::WebhookRubygemsMetadata>>,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "tag_name", skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(rename = "target_commitish")]
    pub target_commitish: String,
    #[serde(rename = "target_oid")]
    pub target_oid: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "version")]
    pub version: String,
}

impl WebhookRegistryPackageUpdatedRegistryPackagePackageVersion {
    pub fn new(author: models::WebhookRegistryPackagePublishedRegistryPackageOwner, body: String, body_html: String, created_at: String, description: String, html_url: String, id: i32, installation_command: String, metadata: Vec<std::collections::HashMap<String, serde_json::Value>>, name: String, package_files: Vec<models::WebhookRegistryPackageUpdatedRegistryPackagePackageVersionPackageFilesInner>, package_url: String, summary: String, target_commitish: String, target_oid: String, updated_at: String, version: String) -> WebhookRegistryPackageUpdatedRegistryPackagePackageVersion {
        WebhookRegistryPackageUpdatedRegistryPackagePackageVersion {
            author: Box::new(author),
            body,
            body_html,
            created_at,
            description,
            docker_metadata: None,
            draft: None,
            html_url,
            id,
            installation_command,
            manifest: None,
            metadata,
            name,
            package_files,
            package_url,
            prerelease: None,
            release: None,
            rubygems_metadata: None,
            summary,
            tag_name: None,
            target_commitish,
            target_oid,
            updated_at,
            version,
        }
    }
}

