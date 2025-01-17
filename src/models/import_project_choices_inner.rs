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
pub struct ImportProjectChoicesInner {
    #[serde(rename = "vcs", skip_serializing_if = "Option::is_none")]
    pub vcs: Option<String>,
    #[serde(rename = "tfvc_project", skip_serializing_if = "Option::is_none")]
    pub tfvc_project: Option<String>,
    #[serde(rename = "human_name", skip_serializing_if = "Option::is_none")]
    pub human_name: Option<String>,
}

impl ImportProjectChoicesInner {
    pub fn new() -> ImportProjectChoicesInner {
        ImportProjectChoicesInner {
            vcs: None,
            tfvc_project: None,
            human_name: None,
        }
    }
}

