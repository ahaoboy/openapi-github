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
pub struct WebhookPullRequestReviewRequestRemovedOneOf {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    /// The pull request number.
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "pull_request")]
    pub pull_request: Box<models::PullRequest6>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "requested_reviewer", deserialize_with = "Option::deserialize")]
    pub requested_reviewer: Option<Box<models::User>>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookPullRequestReviewRequestRemovedOneOf {
    pub fn new(action: Action, number: i32, pull_request: models::PullRequest6, repository: models::RepositoryWebhooks, requested_reviewer: Option<models::User>, sender: models::SimpleUserWebhooks) -> WebhookPullRequestReviewRequestRemovedOneOf {
        WebhookPullRequestReviewRequestRemovedOneOf {
            action,
            enterprise: None,
            installation: None,
            number,
            organization: None,
            pull_request: Box::new(pull_request),
            repository: Box::new(repository),
            requested_reviewer: requested_reviewer.map(Box::new),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "review_request_removed")]
    ReviewRequestRemoved,
}

impl Default for Action {
    fn default() -> Action {
        Self::ReviewRequestRemoved
    }
}

