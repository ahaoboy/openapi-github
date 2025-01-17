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
pub struct UsersCreateGpgKeyForAuthenticatedUserRequest {
    /// A descriptive name for the new key.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A GPG key in ASCII-armored format.
    #[serde(rename = "armored_public_key")]
    pub armored_public_key: String,
}

impl UsersCreateGpgKeyForAuthenticatedUserRequest {
    pub fn new(armored_public_key: String) -> UsersCreateGpgKeyForAuthenticatedUserRequest {
        UsersCreateGpgKeyForAuthenticatedUserRequest {
            name: None,
            armored_public_key,
        }
    }
}

