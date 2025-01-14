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

/// CommunityProfile : Community Profile
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunityProfile {
    #[serde(rename = "health_percentage")]
    pub health_percentage: i32,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "documentation", deserialize_with = "Option::deserialize")]
    pub documentation: Option<String>,
    #[serde(rename = "files")]
    pub files: Box<models::CommunityProfileFiles>,
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
    #[serde(rename = "content_reports_enabled", skip_serializing_if = "Option::is_none")]
    pub content_reports_enabled: Option<bool>,
}

impl CommunityProfile {
    /// Community Profile
    pub fn new(health_percentage: i32, description: Option<String>, documentation: Option<String>, files: models::CommunityProfileFiles, updated_at: Option<String>) -> CommunityProfile {
        CommunityProfile {
            health_percentage,
            description,
            documentation,
            files: Box::new(files),
            updated_at,
            content_reports_enabled: None,
        }
    }
}

