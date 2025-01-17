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

/// OrgsCreateOrUpdateCustomPropertyRequestDefaultValue : Default value of the property
/// Default value of the property
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrgsCreateOrUpdateCustomPropertyRequestDefaultValue {
    String(String),
    Array(Vec<String>),
}

impl Default for OrgsCreateOrUpdateCustomPropertyRequestDefaultValue {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

