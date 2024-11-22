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
pub struct AppsDeleteAuthorizationRequest {
    /// The OAuth access token used to authenticate to the GitHub API.
    #[serde(rename = "access_token")]
    pub access_token: String,
}

impl AppsDeleteAuthorizationRequest {
    pub fn new(access_token: String) -> AppsDeleteAuthorizationRequest {
        AppsDeleteAuthorizationRequest {
            access_token,
        }
    }
}
