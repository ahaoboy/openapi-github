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
pub struct WebhookPullRequestReviewSubmitted {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "pull_request")]
    pub pull_request: Box<models::SimplePullRequest2>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "review")]
    pub review: Box<models::WebhooksReview>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookPullRequestReviewSubmitted {
    pub fn new(action: Action, pull_request: models::SimplePullRequest2, repository: models::RepositoryWebhooks, review: models::WebhooksReview, sender: models::SimpleUserWebhooks) -> WebhookPullRequestReviewSubmitted {
        WebhookPullRequestReviewSubmitted {
            action,
            enterprise: None,
            installation: None,
            organization: None,
            pull_request: Box::new(pull_request),
            repository: Box::new(repository),
            review: Box::new(review),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "submitted")]
    Submitted,
}

impl Default for Action {
    fn default() -> Action {
        Self::Submitted
    }
}
