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
pub struct ActionsReviewCustomGatesForRunRequest {
    /// The name of the environment to approve or reject.
    #[serde(rename = "environment_name")]
    pub environment_name: String,
    /// Optional comment to include with the review.
    #[serde(rename = "comment")]
    pub comment: String,
    /// Whether to approve or reject deployment to the specified environments.
    #[serde(rename = "state")]
    pub state: State,
}

impl ActionsReviewCustomGatesForRunRequest {
    pub fn new(environment_name: String, comment: String, state: State) -> ActionsReviewCustomGatesForRunRequest {
        ActionsReviewCustomGatesForRunRequest {
            environment_name,
            comment,
            state,
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
