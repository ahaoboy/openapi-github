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

/// ReposUpdateBranchProtectionRequestRestrictions : Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReposUpdateBranchProtectionRequestRestrictions {
    /// The list of user `login`s with push access
    #[serde(rename = "users")]
    pub users: Vec<String>,
    /// The list of team `slug`s with push access
    #[serde(rename = "teams")]
    pub teams: Vec<String>,
    /// The list of app `slug`s with push access
    #[serde(rename = "apps", skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<String>>,
}

impl ReposUpdateBranchProtectionRequestRestrictions {
    /// Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
    pub fn new(users: Vec<String>, teams: Vec<String>) -> ReposUpdateBranchProtectionRequestRestrictions {
        ReposUpdateBranchProtectionRequestRestrictions {
            users,
            teams,
            apps: None,
        }
    }
}

