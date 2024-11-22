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
pub struct ReposGenerateReleaseNotesRequest {
    /// The tag name for the release. This can be an existing tag or a new one.
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    /// Specifies the commitish value that will be the target for the release's tag. Required if the supplied tag_name does not reference an existing tag. Ignored if the tag_name already exists.
    #[serde(rename = "target_commitish", skip_serializing_if = "Option::is_none")]
    pub target_commitish: Option<String>,
    /// The name of the previous tag to use as the starting point for the release notes. Use to manually specify the range for the set of changes considered as part this release.
    #[serde(rename = "previous_tag_name", skip_serializing_if = "Option::is_none")]
    pub previous_tag_name: Option<String>,
    /// Specifies a path to a file in the repository containing configuration settings used for generating the release notes. If unspecified, the configuration file located in the repository at '.github/release.yml' or '.github/release.yaml' will be used. If that is not present, the default configuration will be used.
    #[serde(rename = "configuration_file_path", skip_serializing_if = "Option::is_none")]
    pub configuration_file_path: Option<String>,
}

impl ReposGenerateReleaseNotesRequest {
    pub fn new(tag_name: String) -> ReposGenerateReleaseNotesRequest {
        ReposGenerateReleaseNotesRequest {
            tag_name,
            target_commitish: None,
            previous_tag_name: None,
            configuration_file_path: None,
        }
    }
}
