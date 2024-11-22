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

/// RemovedFromProjectIssueEvent : Removed from Project Issue Event
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemovedFromProjectIssueEvent {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "actor")]
    pub actor: Box<models::SimpleUser>,
    #[serde(rename = "event")]
    pub event: String,
    #[serde(rename = "commit_id", deserialize_with = "Option::deserialize")]
    pub commit_id: Option<String>,
    #[serde(rename = "commit_url", deserialize_with = "Option::deserialize")]
    pub commit_url: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "performed_via_github_app", deserialize_with = "Option::deserialize")]
    pub performed_via_github_app: Option<Box<models::NullableIntegration>>,
    #[serde(rename = "project_card", skip_serializing_if = "Option::is_none")]
    pub project_card: Option<Box<models::AddedToProjectIssueEventProjectCard>>,
}

impl RemovedFromProjectIssueEvent {
    /// Removed from Project Issue Event
    pub fn new(id: i32, node_id: String, url: String, actor: models::SimpleUser, event: String, commit_id: Option<String>, commit_url: Option<String>, created_at: String, performed_via_github_app: Option<models::NullableIntegration>) -> RemovedFromProjectIssueEvent {
        RemovedFromProjectIssueEvent {
            id,
            node_id,
            url,
            actor: Box::new(actor),
            event,
            commit_id,
            commit_url,
            created_at,
            performed_via_github_app: performed_via_github_app.map(Box::new),
            project_card: None,
        }
    }
}

