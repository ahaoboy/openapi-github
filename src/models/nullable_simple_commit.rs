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

/// NullableSimpleCommit : A commit.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NullableSimpleCommit {
    /// SHA for the commit
    #[serde(rename = "id")]
    pub id: String,
    /// SHA for the commit's tree
    #[serde(rename = "tree_id")]
    pub tree_id: String,
    /// Message describing the purpose of the commit
    #[serde(rename = "message")]
    pub message: String,
    /// Timestamp of the commit
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "author", deserialize_with = "Option::deserialize")]
    pub author: Option<Box<models::NullableSimpleCommitAuthor>>,
    #[serde(rename = "committer", deserialize_with = "Option::deserialize")]
    pub committer: Option<Box<models::NullableSimpleCommitCommitter>>,
}

impl NullableSimpleCommit {
    /// A commit.
    pub fn new(id: String, tree_id: String, message: String, timestamp: String, author: Option<models::NullableSimpleCommitAuthor>, committer: Option<models::NullableSimpleCommitCommitter>) -> NullableSimpleCommit {
        NullableSimpleCommit {
            id,
            tree_id,
            message,
            timestamp,
            author: author.map(Box::new),
            committer: committer.map(Box::new),
        }
    }
}

