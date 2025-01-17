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
pub struct WorkflowRun2 {
    #[serde(rename = "actor", deserialize_with = "Option::deserialize")]
    pub actor: Option<Box<models::User>>,
    #[serde(rename = "artifacts_url")]
    pub artifacts_url: String,
    #[serde(rename = "cancel_url")]
    pub cancel_url: String,
    #[serde(rename = "check_suite_id")]
    pub check_suite_id: i32,
    #[serde(rename = "check_suite_node_id")]
    pub check_suite_node_id: String,
    #[serde(rename = "check_suite_url")]
    pub check_suite_url: String,
    #[serde(rename = "conclusion", deserialize_with = "Option::deserialize")]
    pub conclusion: Option<Conclusion>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "event")]
    pub event: String,
    #[serde(rename = "head_branch", deserialize_with = "Option::deserialize")]
    pub head_branch: Option<String>,
    #[serde(rename = "head_commit")]
    pub head_commit: Box<models::SimpleCommit>,
    #[serde(rename = "head_repository")]
    pub head_repository: Box<models::RepositoryLite>,
    #[serde(rename = "head_sha")]
    pub head_sha: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "jobs_url")]
    pub jobs_url: String,
    #[serde(rename = "logs_url")]
    pub logs_url: String,
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "previous_attempt_url", deserialize_with = "Option::deserialize")]
    pub previous_attempt_url: Option<String>,
    #[serde(rename = "pull_requests")]
    pub pull_requests: Vec<models::WorkflowRun2PullRequestsInner>,
    #[serde(rename = "referenced_workflows", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub referenced_workflows: Option<Option<Vec<models::DeploymentWorkflowRunReferencedWorkflowsInner>>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryLite>,
    #[serde(rename = "rerun_url")]
    pub rerun_url: String,
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
    #[serde(rename = "workflow_url")]
    pub workflow_url: String,
    #[serde(rename = "display_title")]
    pub display_title: String,
}

impl WorkflowRun2 {
    pub fn new(actor: Option<models::User>, artifacts_url: String, cancel_url: String, check_suite_id: i32, check_suite_node_id: String, check_suite_url: String, conclusion: Option<Conclusion>, created_at: String, event: String, head_branch: Option<String>, head_commit: models::SimpleCommit, head_repository: models::RepositoryLite, head_sha: String, html_url: String, id: i32, jobs_url: String, logs_url: String, name: Option<String>, node_id: String, path: String, previous_attempt_url: Option<String>, pull_requests: Vec<models::WorkflowRun2PullRequestsInner>, repository: models::RepositoryLite, rerun_url: String, run_attempt: i32, run_number: i32, run_started_at: String, status: Status, triggering_actor: Option<models::User>, updated_at: String, url: String, workflow_id: i32, workflow_url: String, display_title: String) -> WorkflowRun2 {
        WorkflowRun2 {
            actor: actor.map(Box::new),
            artifacts_url,
            cancel_url,
            check_suite_id,
            check_suite_node_id,
            check_suite_url,
            conclusion,
            created_at,
            event,
            head_branch,
            head_commit: Box::new(head_commit),
            head_repository: Box::new(head_repository),
            head_sha,
            html_url,
            id,
            jobs_url,
            logs_url,
            name,
            node_id,
            path,
            previous_attempt_url,
            pull_requests,
            referenced_workflows: None,
            repository: Box::new(repository),
            rerun_url,
            run_attempt,
            run_number,
            run_started_at,
            status,
            triggering_actor: triggering_actor.map(Box::new),
            updated_at,
            url,
            workflow_id,
            workflow_url,
            display_title,
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
    #[serde(rename = "skipped")]
    Skipped,
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
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "waiting")]
    Waiting,
}

impl Default for Status {
    fn default() -> Status {
        Self::Requested
    }
}

