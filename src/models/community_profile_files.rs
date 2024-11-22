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
pub struct CommunityProfileFiles {
    #[serde(rename = "code_of_conduct", deserialize_with = "Option::deserialize")]
    pub code_of_conduct: Option<Box<models::NullableCodeOfConductSimple>>,
    #[serde(rename = "code_of_conduct_file", deserialize_with = "Option::deserialize")]
    pub code_of_conduct_file: Option<Box<models::NullableCommunityHealthFile>>,
    #[serde(rename = "license", deserialize_with = "Option::deserialize")]
    pub license: Option<Box<models::NullableLicenseSimple>>,
    #[serde(rename = "contributing", deserialize_with = "Option::deserialize")]
    pub contributing: Option<Box<models::NullableCommunityHealthFile>>,
    #[serde(rename = "readme", deserialize_with = "Option::deserialize")]
    pub readme: Option<Box<models::NullableCommunityHealthFile>>,
    #[serde(rename = "issue_template", deserialize_with = "Option::deserialize")]
    pub issue_template: Option<Box<models::NullableCommunityHealthFile>>,
    #[serde(rename = "pull_request_template", deserialize_with = "Option::deserialize")]
    pub pull_request_template: Option<Box<models::NullableCommunityHealthFile>>,
}

impl CommunityProfileFiles {
    pub fn new(code_of_conduct: Option<models::NullableCodeOfConductSimple>, code_of_conduct_file: Option<models::NullableCommunityHealthFile>, license: Option<models::NullableLicenseSimple>, contributing: Option<models::NullableCommunityHealthFile>, readme: Option<models::NullableCommunityHealthFile>, issue_template: Option<models::NullableCommunityHealthFile>, pull_request_template: Option<models::NullableCommunityHealthFile>) -> CommunityProfileFiles {
        CommunityProfileFiles {
            code_of_conduct: code_of_conduct.map(Box::new),
            code_of_conduct_file: code_of_conduct_file.map(Box::new),
            license: license.map(Box::new),
            contributing: contributing.map(Box::new),
            readme: readme.map(Box::new),
            issue_template: issue_template.map(Box::new),
            pull_request_template: pull_request_template.map(Box::new),
        }
    }
}

