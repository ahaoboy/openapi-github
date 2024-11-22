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

/// GitCommitAuthor : Identifying information for the git-user
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitCommitAuthor {
    /// Timestamp of the commit
    #[serde(rename = "date")]
    pub date: String,
    /// Git email address of the user
    #[serde(rename = "email")]
    pub email: String,
    /// Name of the git user
    #[serde(rename = "name")]
    pub name: String,
}

impl GitCommitAuthor {
    /// Identifying information for the git-user
    pub fn new(date: String, email: String, name: String) -> GitCommitAuthor {
        GitCommitAuthor {
            date,
            email,
            name,
        }
    }
}
