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
pub struct AlertInstance {
    /// Identifies the configuration under which the analysis was executed. For example, in GitHub Actions this includes the workflow filename and job name.
    #[serde(rename = "analysis_key")]
    pub analysis_key: String,
    /// Identifies the configuration under which the analysis was executed.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "classifications", skip_serializing_if = "Option::is_none")]
    pub classifications: Option<Vec<String>>,
    #[serde(rename = "commit_sha", skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    /// Identifies the variable values associated with the environment in which the analysis that generated this alert instance was performed, such as the language that was analyzed.
    #[serde(rename = "environment")]
    pub environment: String,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<models::AlertInstanceLocation>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<models::CodeScanningAlertInstanceMessage>>,
    /// The full Git reference, formatted as `refs/heads/<branch name>`.
    #[serde(rename = "ref")]
    pub r#ref: String,
    /// State of a code scanning alert.
    #[serde(rename = "state")]
    pub state: State,
}

impl AlertInstance {
    pub fn new(analysis_key: String, environment: String, r#ref: String, state: State) -> AlertInstance {
        AlertInstance {
            analysis_key,
            category: None,
            classifications: None,
            commit_sha: None,
            environment,
            location: None,
            message: None,
            r#ref,
            state,
        }
    }
}
/// State of a code scanning alert.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "fixed")]
    Fixed,
}

impl Default for State {
    fn default() -> State {
        Self::Open
    }
}

