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
pub struct FileCommitCommitVerification {
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "signature", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub signature: Option<Option<String>>,
    #[serde(rename = "payload", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Option<String>>,
}

impl FileCommitCommitVerification {
    pub fn new() -> FileCommitCommitVerification {
        FileCommitCommitVerification {
            verified: None,
            reason: None,
            signature: None,
            payload: None,
        }
    }
}

