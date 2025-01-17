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
pub struct TeamsUpdateDiscussionInOrgRequest {
    /// The discussion post's title.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The discussion post's body text.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl TeamsUpdateDiscussionInOrgRequest {
    pub fn new() -> TeamsUpdateDiscussionInOrgRequest {
        TeamsUpdateDiscussionInOrgRequest {
            title: None,
            body: None,
        }
    }
}

