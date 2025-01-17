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
pub struct ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "users_url")]
    pub users_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "users")]
    pub users: Vec<models::SimpleUser>,
    #[serde(rename = "teams")]
    pub teams: Vec<models::Team>,
    #[serde(rename = "apps", skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<models::Integration>>,
}

impl ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions {
    pub fn new(url: String, users_url: String, teams_url: String, users: Vec<models::SimpleUser>, teams: Vec<models::Team>) -> ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions {
        ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions {
            url,
            users_url,
            teams_url,
            users,
            teams,
            apps: None,
        }
    }
}

