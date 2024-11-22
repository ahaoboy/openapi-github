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
pub struct WebhookDeploymentReviewRequested {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "environment")]
    pub environment: String,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimpleWebhooks>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "requestor", deserialize_with = "Option::deserialize")]
    pub requestor: Option<Box<models::WebhooksUser>>,
    #[serde(rename = "reviewers")]
    pub reviewers: Vec<models::WebhookDeploymentReviewRequestedReviewersInner>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
    #[serde(rename = "since")]
    pub since: String,
    #[serde(rename = "workflow_job_run")]
    pub workflow_job_run: Box<models::WebhookDeploymentReviewRequestedWorkflowJobRun>,
    #[serde(rename = "workflow_run", deserialize_with = "Option::deserialize")]
    pub workflow_run: Option<Box<models::DeploymentWorkflowRun3>>,
}

impl WebhookDeploymentReviewRequested {
    pub fn new(action: Action, environment: String, organization: models::OrganizationSimpleWebhooks, repository: models::RepositoryWebhooks, requestor: Option<models::WebhooksUser>, reviewers: Vec<models::WebhookDeploymentReviewRequestedReviewersInner>, sender: models::SimpleUserWebhooks, since: String, workflow_job_run: models::WebhookDeploymentReviewRequestedWorkflowJobRun, workflow_run: Option<models::DeploymentWorkflowRun3>) -> WebhookDeploymentReviewRequested {
        WebhookDeploymentReviewRequested {
            action,
            enterprise: None,
            environment,
            installation: None,
            organization: Box::new(organization),
            repository: Box::new(repository),
            requestor: if let Some(x) = requestor {Some(Box::new(x))} else {None},
            reviewers,
            sender: Box::new(sender),
            since,
            workflow_job_run: Box::new(workflow_job_run),
            workflow_run: if let Some(x) = workflow_run {Some(Box::new(x))} else {None},
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "requested")]
    Requested,
}

impl Default for Action {
    fn default() -> Action {
        Self::Requested
    }
}
