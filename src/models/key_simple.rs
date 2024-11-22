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

/// KeySimple : Key Simple
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeySimple {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "key")]
    pub key: String,
}

impl KeySimple {
    /// Key Simple
    pub fn new(id: i32, key: String) -> KeySimple {
        KeySimple {
            id,
            key,
        }
    }
}
