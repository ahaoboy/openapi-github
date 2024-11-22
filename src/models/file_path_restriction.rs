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

/// FilePathRestriction : Note: file_path_restriction is in beta and subject to change.  Prevent commits that include changes in specified file paths from being pushed to the commit graph.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilePathRestriction {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<models::FilePathRestrictionParameters>>,
}

impl FilePathRestriction {
    /// Note: file_path_restriction is in beta and subject to change.  Prevent commits that include changes in specified file paths from being pushed to the commit graph.
    pub fn new(r#type: Type) -> FilePathRestriction {
        FilePathRestriction {
            r#type,
            parameters: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "file_path_restriction")]
    FilePathRestriction,
}

impl Default for Type {
    fn default() -> Type {
        Self::FilePathRestriction
    }
}
