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
pub struct ActionsReviewPendingDeploymentsForRunRequest {
    /// The list of environment ids to approve or reject
    #[serde(rename = "environment_ids")]
    pub environment_ids: Vec<i32>,
    /// Whether to approve or reject deployment to the specified environments.
    #[serde(rename = "state")]
    pub state: State,
    /// A comment to accompany the deployment review
    #[serde(rename = "comment")]
    pub comment: String,
}

impl ActionsReviewPendingDeploymentsForRunRequest {
    pub fn new(environment_ids: Vec<i32>, state: State, comment: String) -> ActionsReviewPendingDeploymentsForRunRequest {
        ActionsReviewPendingDeploymentsForRunRequest {
            environment_ids,
            state,
            comment,
        }
    }
}
/// Whether to approve or reject deployment to the specified environments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
}

impl Default for State {
    fn default() -> State {
        Self::Approved
    }
}

