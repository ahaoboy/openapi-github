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
pub struct PullRequest10Head {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "repo")]
    pub repo: Box<models::Repository12>,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User>>,
}

impl PullRequest10Head {
    pub fn new(label: String, r#ref: String, repo: models::Repository12, sha: String, user: Option<models::User>) -> PullRequest10Head {
        PullRequest10Head {
            label,
            r#ref,
            repo: Box::new(repo),
            sha,
            user: user.map(Box::new),
        }
    }
}

