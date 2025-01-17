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
pub struct ActionsListSelectedRepositoriesEnabledGithubActionsOrganization200Response {
    #[serde(rename = "total_count")]
    pub total_count: f64,
    #[serde(rename = "repositories")]
    pub repositories: Vec<models::Repository>,
}

impl ActionsListSelectedRepositoriesEnabledGithubActionsOrganization200Response {
    pub fn new(total_count: f64, repositories: Vec<models::Repository>) -> ActionsListSelectedRepositoriesEnabledGithubActionsOrganization200Response {
        ActionsListSelectedRepositoriesEnabledGithubActionsOrganization200Response {
            total_count,
            repositories,
        }
    }
}

