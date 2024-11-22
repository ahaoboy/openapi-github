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

/// IssueSearchResultItem : Issue Search Result Item
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueSearchResultItem {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "repository_url")]
    pub repository_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "locked")]
    pub locked: bool,
    #[serde(rename = "active_lock_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<Option<String>>,
    #[serde(rename = "assignees", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Option<Vec<models::SimpleUser>>>,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "labels")]
    pub labels: Vec<models::IssueSearchResultItemLabelsInner>,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "state_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<Option<String>>,
    #[serde(rename = "assignee", deserialize_with = "Option::deserialize")]
    pub assignee: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "milestone", deserialize_with = "Option::deserialize")]
    pub milestone: Option<Box<models::NullableMilestone>>,
    #[serde(rename = "comments")]
    pub comments: i32,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "closed_at", deserialize_with = "Option::deserialize")]
    pub closed_at: Option<String>,
    #[serde(rename = "text_matches", skip_serializing_if = "Option::is_none")]
    pub text_matches: Option<Vec<models::SearchResultTextMatchesInner>>,
    #[serde(rename = "pull_request", skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<Box<models::IssuePullRequest>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "score")]
    pub score: f64,
    #[serde(rename = "author_association")]
    pub author_association: models::AuthorAssociation,
    #[serde(rename = "draft", skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::Repository>>,
    #[serde(rename = "body_html", skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(rename = "body_text", skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(rename = "timeline_url", skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    #[serde(rename = "performed_via_github_app", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Option<Box<models::NullableIntegration>>>,
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Box<models::ReactionRollup>>,
}

impl IssueSearchResultItem {
    /// Issue Search Result Item
    pub fn new(url: String, repository_url: String, labels_url: String, comments_url: String, events_url: String, html_url: String, id: i64, node_id: String, number: i32, title: String, locked: bool, user: Option<models::NullableSimpleUser>, labels: Vec<models::IssueSearchResultItemLabelsInner>, state: String, assignee: Option<models::NullableSimpleUser>, milestone: Option<models::NullableMilestone>, comments: i32, created_at: String, updated_at: String, closed_at: Option<String>, score: f64, author_association: models::AuthorAssociation) -> IssueSearchResultItem {
        IssueSearchResultItem {
            url,
            repository_url,
            labels_url,
            comments_url,
            events_url,
            html_url,
            id,
            node_id,
            number,
            title,
            locked,
            active_lock_reason: None,
            assignees: None,
            user: user.map(Box::new),
            labels,
            state,
            state_reason: None,
            assignee: assignee.map(Box::new),
            milestone: milestone.map(Box::new),
            comments,
            created_at,
            updated_at,
            closed_at,
            text_matches: None,
            pull_request: None,
            body: None,
            score,
            author_association,
            draft: None,
            repository: None,
            body_html: None,
            body_text: None,
            timeline_url: None,
            performed_via_github_app: None,
            reactions: None,
        }
    }
}

