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
pub struct ActionsWorkflowAccessToRepository {
    /// Defines the level of access that workflows outside of the repository have to actions and reusable workflows within the repository.  `none` means the access is only possible from workflows in this repository. `user` level access allows sharing across user owned private repositories only. `organization` level access allows sharing across the organization.
    #[serde(rename = "access_level")]
    pub access_level: AccessLevel,
}

impl ActionsWorkflowAccessToRepository {
    pub fn new(access_level: AccessLevel) -> ActionsWorkflowAccessToRepository {
        ActionsWorkflowAccessToRepository {
            access_level,
        }
    }
}
/// Defines the level of access that workflows outside of the repository have to actions and reusable workflows within the repository.  `none` means the access is only possible from workflows in this repository. `user` level access allows sharing across user owned private repositories only. `organization` level access allows sharing across the organization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccessLevel {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "organization")]
    Organization,
}

impl Default for AccessLevel {
    fn default() -> AccessLevel {
        Self::None
    }
}
