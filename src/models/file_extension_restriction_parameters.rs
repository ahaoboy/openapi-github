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
pub struct FileExtensionRestrictionParameters {
    /// The file extensions that are restricted from being pushed to the commit graph.
    #[serde(rename = "restricted_file_extensions")]
    pub restricted_file_extensions: Vec<String>,
}

impl FileExtensionRestrictionParameters {
    pub fn new(restricted_file_extensions: Vec<String>) -> FileExtensionRestrictionParameters {
        FileExtensionRestrictionParameters {
            restricted_file_extensions,
        }
    }
}

