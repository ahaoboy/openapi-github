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

/// SocialAccount : Social media account
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialAccount {
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl SocialAccount {
    /// Social media account
    pub fn new(provider: String, url: String) -> SocialAccount {
        SocialAccount {
            provider,
            url,
        }
    }
}

