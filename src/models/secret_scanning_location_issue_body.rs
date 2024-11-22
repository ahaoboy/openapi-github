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

/// SecretScanningLocationIssueBody : Represents an 'issue_body' secret scanning location type. This location type shows that a secret was detected in the body of an issue.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretScanningLocationIssueBody {
    /// The API URL to get the issue where the secret was detected.
    #[serde(rename = "issue_body_url")]
    pub issue_body_url: String,
}

impl SecretScanningLocationIssueBody {
    /// Represents an 'issue_body' secret scanning location type. This location type shows that a secret was detected in the body of an issue.
    pub fn new(issue_body_url: String) -> SecretScanningLocationIssueBody {
        SecretScanningLocationIssueBody {
            issue_body_url,
        }
    }
}
