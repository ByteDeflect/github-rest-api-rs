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
pub struct WebhookRegistryPackagePublishedRegistryPackagePackageVersion {
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::WebhookRegistryPackagePublishedRegistryPackageOwner>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<models::WebhookPackagePublishedPackagePackageVersionBody>>,
    #[serde(rename = "body_html", skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(rename = "container_metadata", skip_serializing_if = "Option::is_none")]
    pub container_metadata: Option<Box<models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionContainerMetadata>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "docker_metadata", skip_serializing_if = "Option::is_none")]
    pub docker_metadata: Option<Vec<models::WebhookPackagePublishedPackagePackageVersionDockerMetadataInner>>,
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
    #[serde(rename = "npm_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub npm_metadata: Option<Option<Box<models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionNpmMetadata>>>,
    #[serde(rename = "nuget_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nuget_metadata: Option<Option<Vec<models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataInner>>>,
    #[serde(rename = "package_files")]
    pub package_files: Vec<models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionPackageFilesInner>,
    #[serde(rename = "package_url")]
    pub package_url: String,
    #[serde(rename = "prerelease", skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(rename = "release", skip_serializing_if = "Option::is_none")]
    pub release: Option<Box<models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionRelease>>,
    #[serde(rename = "rubygems_metadata", skip_serializing_if = "Option::is_none")]
    pub rubygems_metadata: Option<Vec<models::WebhookRubygemsMetadata>>,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "tag_name", skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(rename = "target_commitish", skip_serializing_if = "Option::is_none")]
    pub target_commitish: Option<String>,
    #[serde(rename = "target_oid", skip_serializing_if = "Option::is_none")]
    pub target_oid: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "version")]
    pub version: String,
}

impl WebhookRegistryPackagePublishedRegistryPackagePackageVersion {
    pub fn new(description: String, html_url: String, id: i32, installation_command: String, metadata: Vec<std::collections::HashMap<String, serde_json::Value>>, name: String, package_files: Vec<models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionPackageFilesInner>, package_url: String, summary: String, version: String) -> WebhookRegistryPackagePublishedRegistryPackagePackageVersion {
        WebhookRegistryPackagePublishedRegistryPackagePackageVersion {
            author: None,
            body: None,
            body_html: None,
            container_metadata: None,
            created_at: None,
            description,
            docker_metadata: None,
            draft: None,
            html_url,
            id,
            installation_command,
            manifest: None,
            metadata,
            name,
            npm_metadata: None,
            nuget_metadata: None,
            package_files,
            package_url,
            prerelease: None,
            release: None,
            rubygems_metadata: None,
            summary,
            tag_name: None,
            target_commitish: None,
            target_oid: None,
            updated_at: None,
            version,
        }
    }
}

