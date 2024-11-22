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

/// NullableMilestone : A collection of related issues and pull requests.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NullableMilestone {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// The number of the milestone.
    #[serde(rename = "number")]
    pub number: i32,
    /// The state of the milestone.
    #[serde(rename = "state")]
    pub state: State,
    /// The title of the milestone.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "creator", deserialize_with = "Option::deserialize")]
    pub creator: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "open_issues")]
    pub open_issues: i32,
    #[serde(rename = "closed_issues")]
    pub closed_issues: i32,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "closed_at", deserialize_with = "Option::deserialize")]
    pub closed_at: Option<String>,
    #[serde(rename = "due_on", deserialize_with = "Option::deserialize")]
    pub due_on: Option<String>,
}

impl NullableMilestone {
    /// A collection of related issues and pull requests.
    pub fn new(url: String, html_url: String, labels_url: String, id: i32, node_id: String, number: i32, state: State, title: String, description: Option<String>, creator: Option<models::NullableSimpleUser>, open_issues: i32, closed_issues: i32, created_at: String, updated_at: String, closed_at: Option<String>, due_on: Option<String>) -> NullableMilestone {
        NullableMilestone {
            url,
            html_url,
            labels_url,
            id,
            node_id,
            number,
            state,
            title,
            description,
            creator: creator.map(Box::new),
            open_issues,
            closed_issues,
            created_at,
            updated_at,
            closed_at,
            due_on,
        }
    }
}
/// The state of the milestone.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

impl Default for State {
    fn default() -> State {
        Self::Open
    }
}

