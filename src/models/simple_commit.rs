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
pub struct SimpleCommit {
    #[serde(rename = "author")]
    pub author: Box<models::Committer>,
    #[serde(rename = "committer")]
    pub committer: Box<models::Committer>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "tree_id")]
    pub tree_id: String,
}

impl SimpleCommit {
    pub fn new(author: models::Committer, committer: models::Committer, id: String, message: String, timestamp: String, tree_id: String) -> SimpleCommit {
        SimpleCommit {
            author: Box::new(author),
            committer: Box::new(committer),
            id,
            message,
            timestamp,
            tree_id,
        }
    }
}

