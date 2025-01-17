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
pub struct RepositoryRulePullRequestParameters {
    /// New, reviewable commits pushed will dismiss previous pull request review approvals.
    #[serde(rename = "dismiss_stale_reviews_on_push")]
    pub dismiss_stale_reviews_on_push: bool,
    /// Require an approving review in pull requests that modify files that have a designated code owner.
    #[serde(rename = "require_code_owner_review")]
    pub require_code_owner_review: bool,
    /// Whether the most recent reviewable push must be approved by someone other than the person who pushed it.
    #[serde(rename = "require_last_push_approval")]
    pub require_last_push_approval: bool,
    /// The number of approving reviews that are required before a pull request can be merged.
    #[serde(rename = "required_approving_review_count")]
    pub required_approving_review_count: i32,
    /// All conversations on code must be resolved before a pull request can be merged.
    #[serde(rename = "required_review_thread_resolution")]
    pub required_review_thread_resolution: bool,
}

impl RepositoryRulePullRequestParameters {
    pub fn new(dismiss_stale_reviews_on_push: bool, require_code_owner_review: bool, require_last_push_approval: bool, required_approving_review_count: i32, required_review_thread_resolution: bool) -> RepositoryRulePullRequestParameters {
        RepositoryRulePullRequestParameters {
            dismiss_stale_reviews_on_push,
            require_code_owner_review,
            require_last_push_approval,
            required_approving_review_count,
            required_review_thread_resolution,
        }
    }
}

