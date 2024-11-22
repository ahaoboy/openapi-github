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
pub struct SimplePullRequest3Head {
    #[serde(rename = "label", deserialize_with = "Option::deserialize")]
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "repo", deserialize_with = "Option::deserialize")]
    pub repo: Option<Box<models::Repository11>>,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User>>,
}

impl SimplePullRequest3Head {
    pub fn new(label: Option<String>, r#ref: String, repo: Option<models::Repository11>, sha: String, user: Option<models::User>) -> SimplePullRequest3Head {
        SimplePullRequest3Head {
            label,
            r#ref,
            repo: if let Some(x) = repo {Some(Box::new(x))} else {None},
            sha,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
        }
    }
}
