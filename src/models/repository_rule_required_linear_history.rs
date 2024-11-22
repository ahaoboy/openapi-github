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

/// RepositoryRuleRequiredLinearHistory : Prevent merge commits from being pushed to matching refs.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRuleRequiredLinearHistory {
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl RepositoryRuleRequiredLinearHistory {
    /// Prevent merge commits from being pushed to matching refs.
    pub fn new(r#type: Type) -> RepositoryRuleRequiredLinearHistory {
        RepositoryRuleRequiredLinearHistory {
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "required_linear_history")]
    RequiredLinearHistory,
}

impl Default for Type {
    fn default() -> Type {
        Self::RequiredLinearHistory
    }
}
