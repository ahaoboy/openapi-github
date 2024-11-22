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
pub struct PullRequest12Head {
    #[serde(rename = "label", deserialize_with = "Option::deserialize")]
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "repo", deserialize_with = "Option::deserialize")]
    pub repo: Option<Box<models::Repository13>>,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User>>,
}

impl PullRequest12Head {
    pub fn new(label: Option<String>, r#ref: String, repo: Option<models::Repository13>, sha: String, user: Option<models::User>) -> PullRequest12Head {
        PullRequest12Head {
            label,
            r#ref,
            repo: repo.map(Box::new),
            sha,
            user: user.map(Box::new),
        }
    }
}

