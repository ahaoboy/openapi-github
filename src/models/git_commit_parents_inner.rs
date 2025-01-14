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
pub struct GitCommitParentsInner {
    /// SHA for the commit
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
}

impl GitCommitParentsInner {
    pub fn new(sha: String, url: String, html_url: String) -> GitCommitParentsInner {
        GitCommitParentsInner {
            sha,
            url,
            html_url,
        }
    }
}

