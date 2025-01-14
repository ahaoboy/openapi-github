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

/// SecretScanningLocationPullRequestReviewComment : Represents a 'pull_request_review_comment' secret scanning location type. This location type shows that a secret was detected in a review comment on a pull request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretScanningLocationPullRequestReviewComment {
    /// The API URL to get the pull request review comment where the secret was detected.
    #[serde(rename = "pull_request_review_comment_url")]
    pub pull_request_review_comment_url: String,
}

impl SecretScanningLocationPullRequestReviewComment {
    /// Represents a 'pull_request_review_comment' secret scanning location type. This location type shows that a secret was detected in a review comment on a pull request.
    pub fn new(pull_request_review_comment_url: String) -> SecretScanningLocationPullRequestReviewComment {
        SecretScanningLocationPullRequestReviewComment {
            pull_request_review_comment_url,
        }
    }
}

