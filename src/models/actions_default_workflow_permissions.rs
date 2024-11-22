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

/// ActionsDefaultWorkflowPermissions : The default workflow permissions granted to the GITHUB_TOKEN when running workflows.
/// The default workflow permissions granted to the GITHUB_TOKEN when running workflows.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionsDefaultWorkflowPermissions {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,

}

impl std::fmt::Display for ActionsDefaultWorkflowPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Read => write!(f, "read"),
            Self::Write => write!(f, "write"),
        }
    }
}

impl Default for ActionsDefaultWorkflowPermissions {
    fn default() -> ActionsDefaultWorkflowPermissions {
        Self::Read
    }
}
