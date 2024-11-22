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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecurityAdvisoriesListGlobalAdvisoriesAffectsParameter {
    String(String),
    Array(Vec<String>),
}

impl Default for SecurityAdvisoriesListGlobalAdvisoriesAffectsParameter {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

impl std::fmt::Display for SecurityAdvisoriesListGlobalAdvisoriesAffectsParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::String(s) => f.write_str(s),
            Self::Array(v) => f.write_str(&v.join(",")),
        }
    }
}
