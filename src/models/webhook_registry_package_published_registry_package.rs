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
pub struct WebhookRegistryPackagePublishedRegistryPackage {
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<String>,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "ecosystem")]
    pub ecosystem: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "owner")]
    pub owner: Box<models::WebhookRegistryPackagePublishedRegistryPackageOwner>,
    #[serde(rename = "package_type")]
    pub package_type: String,
    #[serde(rename = "package_version", deserialize_with = "Option::deserialize")]
    pub package_version: Option<Box<models::WebhookRegistryPackagePublishedRegistryPackagePackageVersion>>,
    #[serde(rename = "registry", deserialize_with = "Option::deserialize")]
    pub registry: Option<Box<models::WebhookRegistryPackagePublishedRegistryPackageRegistry>>,
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
}

impl WebhookRegistryPackagePublishedRegistryPackage {
    pub fn new(created_at: Option<String>, description: Option<String>, ecosystem: String, html_url: String, id: i32, name: String, namespace: String, owner: models::WebhookRegistryPackagePublishedRegistryPackageOwner, package_type: String, package_version: Option<models::WebhookRegistryPackagePublishedRegistryPackagePackageVersion>, registry: Option<models::WebhookRegistryPackagePublishedRegistryPackageRegistry>, updated_at: Option<String>) -> WebhookRegistryPackagePublishedRegistryPackage {
        WebhookRegistryPackagePublishedRegistryPackage {
            created_at,
            description,
            ecosystem,
            html_url,
            id,
            name,
            namespace,
            owner: Box::new(owner),
            package_type,
            package_version: if let Some(x) = package_version {Some(Box::new(x))} else {None},
            registry: if let Some(x) = registry {Some(Box::new(x))} else {None},
            updated_at,
        }
    }
}

