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

/// App1Permissions : The set of permissions for the GitHub app
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct App1Permissions {
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Actions>,
    #[serde(rename = "administration", skip_serializing_if = "Option::is_none")]
    pub administration: Option<Administration>,
    #[serde(rename = "checks", skip_serializing_if = "Option::is_none")]
    pub checks: Option<Checks>,
    #[serde(rename = "content_references", skip_serializing_if = "Option::is_none")]
    pub content_references: Option<ContentReferences>,
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<Contents>,
    #[serde(rename = "deployments", skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Deployments>,
    #[serde(rename = "discussions", skip_serializing_if = "Option::is_none")]
    pub discussions: Option<Discussions>,
    #[serde(rename = "emails", skip_serializing_if = "Option::is_none")]
    pub emails: Option<Emails>,
    #[serde(rename = "environments", skip_serializing_if = "Option::is_none")]
    pub environments: Option<Environments>,
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Issues>,
    #[serde(rename = "keys", skip_serializing_if = "Option::is_none")]
    pub keys: Option<Keys>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Members>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "organization_administration", skip_serializing_if = "Option::is_none")]
    pub organization_administration: Option<OrganizationAdministration>,
    #[serde(rename = "organization_hooks", skip_serializing_if = "Option::is_none")]
    pub organization_hooks: Option<OrganizationHooks>,
    #[serde(rename = "organization_packages", skip_serializing_if = "Option::is_none")]
    pub organization_packages: Option<OrganizationPackages>,
    #[serde(rename = "organization_plan", skip_serializing_if = "Option::is_none")]
    pub organization_plan: Option<OrganizationPlan>,
    #[serde(rename = "organization_projects", skip_serializing_if = "Option::is_none")]
    pub organization_projects: Option<OrganizationProjects>,
    #[serde(rename = "organization_secrets", skip_serializing_if = "Option::is_none")]
    pub organization_secrets: Option<OrganizationSecrets>,
    #[serde(rename = "organization_self_hosted_runners", skip_serializing_if = "Option::is_none")]
    pub organization_self_hosted_runners: Option<OrganizationSelfHostedRunners>,
    #[serde(rename = "organization_user_blocking", skip_serializing_if = "Option::is_none")]
    pub organization_user_blocking: Option<OrganizationUserBlocking>,
    #[serde(rename = "packages", skip_serializing_if = "Option::is_none")]
    pub packages: Option<Packages>,
    #[serde(rename = "pages", skip_serializing_if = "Option::is_none")]
    pub pages: Option<Pages>,
    #[serde(rename = "pull_requests", skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<PullRequests>,
    #[serde(rename = "repository_hooks", skip_serializing_if = "Option::is_none")]
    pub repository_hooks: Option<RepositoryHooks>,
    #[serde(rename = "repository_projects", skip_serializing_if = "Option::is_none")]
    pub repository_projects: Option<RepositoryProjects>,
    #[serde(rename = "secret_scanning_alerts", skip_serializing_if = "Option::is_none")]
    pub secret_scanning_alerts: Option<SecretScanningAlerts>,
    #[serde(rename = "secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Secrets>,
    #[serde(rename = "security_events", skip_serializing_if = "Option::is_none")]
    pub security_events: Option<SecurityEvents>,
    #[serde(rename = "security_scanning_alert", skip_serializing_if = "Option::is_none")]
    pub security_scanning_alert: Option<SecurityScanningAlert>,
    #[serde(rename = "single_file", skip_serializing_if = "Option::is_none")]
    pub single_file: Option<SingleFile>,
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Statuses>,
    #[serde(rename = "team_discussions", skip_serializing_if = "Option::is_none")]
    pub team_discussions: Option<TeamDiscussions>,
    #[serde(rename = "vulnerability_alerts", skip_serializing_if = "Option::is_none")]
    pub vulnerability_alerts: Option<VulnerabilityAlerts>,
    #[serde(rename = "workflows", skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Workflows>,
}

impl App1Permissions {
    /// The set of permissions for the GitHub app
    pub fn new() -> App1Permissions {
        App1Permissions {
            actions: None,
            administration: None,
            checks: None,
            content_references: None,
            contents: None,
            deployments: None,
            discussions: None,
            emails: None,
            environments: None,
            issues: None,
            keys: None,
            members: None,
            metadata: None,
            organization_administration: None,
            organization_hooks: None,
            organization_packages: None,
            organization_plan: None,
            organization_projects: None,
            organization_secrets: None,
            organization_self_hosted_runners: None,
            organization_user_blocking: None,
            packages: None,
            pages: None,
            pull_requests: None,
            repository_hooks: None,
            repository_projects: None,
            secret_scanning_alerts: None,
            secrets: None,
            security_events: None,
            security_scanning_alert: None,
            single_file: None,
            statuses: None,
            team_discussions: None,
            vulnerability_alerts: None,
            workflows: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Actions {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Actions {
    fn default() -> Actions {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Administration {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Administration {
    fn default() -> Administration {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Checks {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Checks {
    fn default() -> Checks {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentReferences {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for ContentReferences {
    fn default() -> ContentReferences {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Contents {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Contents {
    fn default() -> Contents {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Deployments {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Deployments {
    fn default() -> Deployments {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Discussions {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Discussions {
    fn default() -> Discussions {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Emails {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Emails {
    fn default() -> Emails {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Environments {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Environments {
    fn default() -> Environments {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Issues {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Issues {
    fn default() -> Issues {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Keys {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Keys {
    fn default() -> Keys {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Members {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Members {
    fn default() -> Members {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metadata {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Metadata {
    fn default() -> Metadata {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrganizationAdministration {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for OrganizationAdministration {
    fn default() -> OrganizationAdministration {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrganizationHooks {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for OrganizationHooks {
    fn default() -> OrganizationHooks {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrganizationPackages {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for OrganizationPackages {
    fn default() -> OrganizationPackages {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrganizationPlan {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for OrganizationPlan {
    fn default() -> OrganizationPlan {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrganizationProjects {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for OrganizationProjects {
    fn default() -> OrganizationProjects {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrganizationSecrets {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for OrganizationSecrets {
    fn default() -> OrganizationSecrets {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrganizationSelfHostedRunners {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for OrganizationSelfHostedRunners {
    fn default() -> OrganizationSelfHostedRunners {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrganizationUserBlocking {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for OrganizationUserBlocking {
    fn default() -> OrganizationUserBlocking {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Packages {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Packages {
    fn default() -> Packages {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Pages {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Pages {
    fn default() -> Pages {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PullRequests {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for PullRequests {
    fn default() -> PullRequests {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RepositoryHooks {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for RepositoryHooks {
    fn default() -> RepositoryHooks {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RepositoryProjects {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for RepositoryProjects {
    fn default() -> RepositoryProjects {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecretScanningAlerts {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for SecretScanningAlerts {
    fn default() -> SecretScanningAlerts {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Secrets {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Secrets {
    fn default() -> Secrets {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityEvents {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for SecurityEvents {
    fn default() -> SecurityEvents {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityScanningAlert {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for SecurityScanningAlert {
    fn default() -> SecurityScanningAlert {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SingleFile {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for SingleFile {
    fn default() -> SingleFile {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Statuses {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Statuses {
    fn default() -> Statuses {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TeamDiscussions {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for TeamDiscussions {
    fn default() -> TeamDiscussions {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VulnerabilityAlerts {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for VulnerabilityAlerts {
    fn default() -> VulnerabilityAlerts {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Workflows {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl Default for Workflows {
    fn default() -> Workflows {
        Self::Read
    }
}

