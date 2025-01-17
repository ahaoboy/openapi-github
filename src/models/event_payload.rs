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
pub struct EventPayload {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "issue", skip_serializing_if = "Option::is_none")]
    pub issue: Option<Box<models::Issue>>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Box<models::IssueComment>>,
    #[serde(rename = "pages", skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<models::EventPayloadPagesInner>>,
}

impl EventPayload {
    pub fn new() -> EventPayload {
        EventPayload {
            action: None,
            issue: None,
            comment: None,
            pages: None,
        }
    }
}

