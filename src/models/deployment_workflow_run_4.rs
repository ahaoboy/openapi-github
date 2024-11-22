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
pub struct DeploymentWorkflowRun4 {
    #[serde(rename = "actor", deserialize_with = "Option::deserialize")]
    pub actor: Option<Box<models::User>>,
    #[serde(rename = "artifacts_url", skip_serializing_if = "Option::is_none")]
    pub artifacts_url: Option<String>,
    #[serde(rename = "cancel_url", skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(rename = "check_suite_id")]
    pub check_suite_id: i32,
    #[serde(rename = "check_suite_node_id")]
    pub check_suite_node_id: String,
    #[serde(rename = "check_suite_url", skip_serializing_if = "Option::is_none")]
    pub check_suite_url: Option<String>,
    #[serde(rename = "conclusion", deserialize_with = "Option::deserialize")]
    pub conclusion: Option<Conclusion>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "display_title")]
    pub display_title: String,
    #[serde(rename = "event")]
    pub event: String,
    #[serde(rename = "head_branch")]
    pub head_branch: String,
    #[serde(rename = "head_commit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub head_commit: Option<Option<serde_json::Value>>,
    #[serde(rename = "head_repository", skip_serializing_if = "Option::is_none")]
    pub head_repository: Option<Box<models::DeploymentWorkflowRunHeadRepository>>,
    #[serde(rename = "head_sha")]
    pub head_sha: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "jobs_url", skip_serializing_if = "Option::is_none")]
    pub jobs_url: Option<String>,
    #[serde(rename = "logs_url", skip_serializing_if = "Option::is_none")]
    pub logs_url: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "previous_attempt_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub previous_attempt_url: Option<Option<serde_json::Value>>,
    #[serde(rename = "pull_requests")]
    pub pull_requests: Vec<models::CheckRunPullRequest>,
    #[serde(rename = "referenced_workflows", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub referenced_workflows: Option<Option<Vec<models::DeploymentWorkflowRunReferencedWorkflowsInner>>>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::DeploymentWorkflowRunHeadRepository>>,
    #[serde(rename = "rerun_url", skip_serializing_if = "Option::is_none")]
    pub rerun_url: Option<String>,
    #[serde(rename = "run_attempt")]
    pub run_attempt: i32,
    #[serde(rename = "run_number")]
    pub run_number: i32,
    #[serde(rename = "run_started_at")]
    pub run_started_at: String,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "triggering_actor", deserialize_with = "Option::deserialize")]
    pub triggering_actor: Option<Box<models::User>>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "workflow_id")]
    pub workflow_id: i32,
    #[serde(rename = "workflow_url", skip_serializing_if = "Option::is_none")]
    pub workflow_url: Option<String>,
}

impl DeploymentWorkflowRun4 {
    pub fn new(actor: Option<models::User>, check_suite_id: i32, check_suite_node_id: String, conclusion: Option<Conclusion>, created_at: String, display_title: String, event: String, head_branch: String, head_sha: String, html_url: String, id: i32, name: String, node_id: String, path: String, pull_requests: Vec<models::CheckRunPullRequest>, run_attempt: i32, run_number: i32, run_started_at: String, status: Status, triggering_actor: Option<models::User>, updated_at: String, url: String, workflow_id: i32) -> DeploymentWorkflowRun4 {
        DeploymentWorkflowRun4 {
            actor: actor.map(Box::new),
            artifacts_url: None,
            cancel_url: None,
            check_suite_id,
            check_suite_node_id,
            check_suite_url: None,
            conclusion,
            created_at,
            display_title,
            event,
            head_branch,
            head_commit: None,
            head_repository: None,
            head_sha,
            html_url,
            id,
            jobs_url: None,
            logs_url: None,
            name,
            node_id,
            path,
            previous_attempt_url: None,
            pull_requests,
            referenced_workflows: None,
            repository: None,
            rerun_url: None,
            run_attempt,
            run_number,
            run_started_at,
            status,
            triggering_actor: triggering_actor.map(Box::new),
            updated_at,
            url,
            workflow_id,
            workflow_url: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Conclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "startup_failure")]
    StartupFailure,
}

impl Default for Conclusion {
    fn default() -> Conclusion {
        Self::Success
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "pending")]
    Pending,
}

impl Default for Status {
    fn default() -> Status {
        Self::Requested
    }
}

