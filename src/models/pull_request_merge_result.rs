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

/// PullRequestMergeResult : Pull Request Merge Result
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMergeResult {
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "merged")]
    pub merged: bool,
    #[serde(rename = "message")]
    pub message: String,
}

impl PullRequestMergeResult {
    /// Pull Request Merge Result
    pub fn new(sha: String, merged: bool, message: String) -> PullRequestMergeResult {
        PullRequestMergeResult {
            sha,
            merged,
            message,
        }
    }
}

