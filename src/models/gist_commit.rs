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

/// GistCommit : Gist Commit
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GistCommit {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "change_status")]
    pub change_status: Box<models::GistHistoryChangeStatus>,
    #[serde(rename = "committed_at")]
    pub committed_at: String,
}

impl GistCommit {
    /// Gist Commit
    pub fn new(url: String, version: String, user: Option<models::NullableSimpleUser>, change_status: models::GistHistoryChangeStatus, committed_at: String) -> GistCommit {
        GistCommit {
            url,
            version,
            user: user.map(Box::new),
            change_status: Box::new(change_status),
            committed_at,
        }
    }
}

