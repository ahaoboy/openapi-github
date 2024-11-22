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

/// CheckRunWithSimpleCheckSuite : A check performed on the code of a given code change
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckRunWithSimpleCheckSuite {
    #[serde(rename = "app", deserialize_with = "Option::deserialize")]
    pub app: Option<Box<models::NullableIntegration>>,
    #[serde(rename = "check_suite")]
    pub check_suite: Box<models::SimpleCheckSuite>,
    #[serde(rename = "completed_at", deserialize_with = "Option::deserialize")]
    pub completed_at: Option<String>,
    #[serde(rename = "conclusion", deserialize_with = "Option::deserialize")]
    pub conclusion: Option<Conclusion>,
    #[serde(rename = "deployment", skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Box<models::DeploymentSimple>>,
    #[serde(rename = "details_url")]
    pub details_url: String,
    #[serde(rename = "external_id")]
    pub external_id: String,
    /// The SHA of the commit that is being checked.
    #[serde(rename = "head_sha")]
    pub head_sha: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    /// The id of the check.
    #[serde(rename = "id")]
    pub id: i32,
    /// The name of the check.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "output")]
    pub output: Box<models::CheckRunWithSimpleCheckSuiteOutput>,
    #[serde(rename = "pull_requests")]
    pub pull_requests: Vec<models::PullRequestMinimal>,
    #[serde(rename = "started_at")]
    pub started_at: String,
    /// The phase of the lifecycle that the check is currently in.
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "url")]
    pub url: String,
}

impl CheckRunWithSimpleCheckSuite {
    /// A check performed on the code of a given code change
    pub fn new(app: Option<models::NullableIntegration>, check_suite: models::SimpleCheckSuite, completed_at: Option<String>, conclusion: Option<Conclusion>, details_url: String, external_id: String, head_sha: String, html_url: String, id: i32, name: String, node_id: String, output: models::CheckRunWithSimpleCheckSuiteOutput, pull_requests: Vec<models::PullRequestMinimal>, started_at: String, status: Status, url: String) -> CheckRunWithSimpleCheckSuite {
        CheckRunWithSimpleCheckSuite {
            app: if let Some(x) = app {Some(Box::new(x))} else {None},
            check_suite: Box::new(check_suite),
            completed_at,
            conclusion,
            deployment: None,
            details_url,
            external_id,
            head_sha,
            html_url,
            id,
            name,
            node_id,
            output: Box::new(output),
            pull_requests,
            started_at,
            status,
            url,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Conclusion {
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "startup_failure")]
    StartupFailure,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
}

impl Default for Conclusion {
    fn default() -> Conclusion {
        Self::Waiting
    }
}
/// The phase of the lifecycle that the check is currently in.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "pending")]
    Pending,
}

impl Default for Status {
    fn default() -> Status {
        Self::Queued
    }
}
