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
pub struct PackageVersionMetadata {
    #[serde(rename = "package_type")]
    pub package_type: PackageType,
    #[serde(rename = "container", skip_serializing_if = "Option::is_none")]
    pub container: Option<Box<models::ContainerMetadata>>,
    #[serde(rename = "docker", skip_serializing_if = "Option::is_none")]
    pub docker: Option<Box<models::DockerMetadata>>,
}

impl PackageVersionMetadata {
    pub fn new(package_type: PackageType) -> PackageVersionMetadata {
        PackageVersionMetadata {
            package_type,
            container: None,
            docker: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PackageType {
    #[serde(rename = "npm")]
    Npm,
    #[serde(rename = "maven")]
    Maven,
    #[serde(rename = "rubygems")]
    Rubygems,
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "nuget")]
    Nuget,
    #[serde(rename = "container")]
    Container,
}

impl Default for PackageType {
    fn default() -> PackageType {
        Self::Npm
    }
}

