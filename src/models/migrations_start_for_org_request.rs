/*
 * GitHub's official OpenAPI spec + Octokit extension
 *
 * OpenAPI specs from https://github.com/github/rest-api-description with the 'x-octokit' extension required by the Octokit SDKs
 *
 * The version of the OpenAPI document: 16.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationsStartForOrgRequest {
    /// A list of arrays indicating which repositories should be migrated.
    #[serde(rename = "repositories")]
    pub repositories: Vec<String>,
    /// Indicates whether repositories should be locked (to prevent manipulation) while migrating data.
    #[serde(rename = "lock_repositories", skip_serializing_if = "Option::is_none")]
    pub lock_repositories: Option<bool>,
    /// Indicates whether metadata should be excluded and only git source should be included for the migration.
    #[serde(rename = "exclude_metadata", skip_serializing_if = "Option::is_none")]
    pub exclude_metadata: Option<bool>,
    /// Indicates whether the repository git data should be excluded from the migration.
    #[serde(rename = "exclude_git_data", skip_serializing_if = "Option::is_none")]
    pub exclude_git_data: Option<bool>,
    /// Indicates whether attachments should be excluded from the migration (to reduce migration archive file size).
    #[serde(rename = "exclude_attachments", skip_serializing_if = "Option::is_none")]
    pub exclude_attachments: Option<bool>,
    /// Indicates whether releases should be excluded from the migration (to reduce migration archive file size).
    #[serde(rename = "exclude_releases", skip_serializing_if = "Option::is_none")]
    pub exclude_releases: Option<bool>,
    /// Indicates whether projects owned by the organization or users should be excluded. from the migration.
    #[serde(rename = "exclude_owner_projects", skip_serializing_if = "Option::is_none")]
    pub exclude_owner_projects: Option<bool>,
    /// Indicates whether this should only include organization metadata (repositories array should be empty and will ignore other flags).
    #[serde(rename = "org_metadata_only", skip_serializing_if = "Option::is_none")]
    pub org_metadata_only: Option<bool>,
    /// Exclude related items from being returned in the response in order to improve performance of the request.
    #[serde(rename = "exclude", skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<Exclude>>,
}

impl MigrationsStartForOrgRequest {
    pub fn new(repositories: Vec<String>) -> MigrationsStartForOrgRequest {
        MigrationsStartForOrgRequest {
            repositories,
            lock_repositories: None,
            exclude_metadata: None,
            exclude_git_data: None,
            exclude_attachments: None,
            exclude_releases: None,
            exclude_owner_projects: None,
            org_metadata_only: None,
            exclude: None,
        }
    }
}
/// Exclude related items from being returned in the response in order to improve performance of the request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Exclude {
    #[serde(rename = "repositories")]
    Repositories,
}

impl Default for Exclude {
    fn default() -> Exclude {
        Self::Repositories
    }
}
