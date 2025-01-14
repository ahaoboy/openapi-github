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

/// SimpleClassroomRepository : A GitHub repository view for Classroom
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleClassroomRepository {
    /// A unique identifier of the repository.
    #[serde(rename = "id")]
    pub id: i32,
    /// The full, globally unique name of the repository.
    #[serde(rename = "full_name")]
    pub full_name: String,
    /// The URL to view the repository on GitHub.com.
    #[serde(rename = "html_url")]
    pub html_url: String,
    /// The GraphQL identifier of the repository.
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// Whether the repository is private.
    #[serde(rename = "private")]
    pub private: bool,
    /// The default branch for the repository.
    #[serde(rename = "default_branch")]
    pub default_branch: String,
}

impl SimpleClassroomRepository {
    /// A GitHub repository view for Classroom
    pub fn new(id: i32, full_name: String, html_url: String, node_id: String, private: bool, default_branch: String) -> SimpleClassroomRepository {
        SimpleClassroomRepository {
            id,
            full_name,
            html_url,
            node_id,
            private,
            default_branch,
        }
    }
}

