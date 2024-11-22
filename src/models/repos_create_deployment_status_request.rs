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
pub struct ReposCreateDeploymentStatusRequest {
    /// The state of the status. When you set a transient deployment to `inactive`, the deployment will be shown as `destroyed` in GitHub.
    #[serde(rename = "state")]
    pub state: State,
    /// The target URL to associate with this status. This URL should contain output to keep the user updated while the task is running or serve as historical information for what happened in the deployment. **Note:** It's recommended to use the `log_url` parameter, which replaces `target_url`.
    #[serde(rename = "target_url", skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    /// The full URL of the deployment's output. This parameter replaces `target_url`. We will continue to accept `target_url` to support legacy uses, but we recommend replacing `target_url` with `log_url`. Setting `log_url` will automatically set `target_url` to the same value. Default: `\"\"`
    #[serde(rename = "log_url", skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    /// A short description of the status. The maximum description length is 140 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Name for the target deployment environment, which can be changed when setting a deploy status. For example, `production`, `staging`, or `qa`. If not defined, the environment of the previous status on the deployment will be used, if it exists. Otherwise, the environment of the deployment will be used.
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Sets the URL for accessing your environment. Default: `\"\"`
    #[serde(rename = "environment_url", skip_serializing_if = "Option::is_none")]
    pub environment_url: Option<String>,
    /// Adds a new `inactive` status to all prior non-transient, non-production environment deployments with the same repository and `environment` name as the created status's deployment. An `inactive` status is only added to deployments that had a `success` state. Default: `true`
    #[serde(rename = "auto_inactive", skip_serializing_if = "Option::is_none")]
    pub auto_inactive: Option<bool>,
}

impl ReposCreateDeploymentStatusRequest {
    pub fn new(state: State) -> ReposCreateDeploymentStatusRequest {
        ReposCreateDeploymentStatusRequest {
            state,
            target_url: None,
            log_url: None,
            description: None,
            environment: None,
            environment_url: None,
            auto_inactive: None,
        }
    }
}
/// The state of the status. When you set a transient deployment to `inactive`, the deployment will be shown as `destroyed` in GitHub.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
}

impl Default for State {
    fn default() -> State {
        Self::Error
    }
}
