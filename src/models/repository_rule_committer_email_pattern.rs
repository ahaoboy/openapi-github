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

/// RepositoryRuleCommitterEmailPattern : Parameters to be used for the committer_email_pattern rule
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRuleCommitterEmailPattern {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<models::RepositoryRuleCommitMessagePatternParameters>>,
}

impl RepositoryRuleCommitterEmailPattern {
    /// Parameters to be used for the committer_email_pattern rule
    pub fn new(r#type: Type) -> RepositoryRuleCommitterEmailPattern {
        RepositoryRuleCommitterEmailPattern {
            r#type,
            parameters: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "committer_email_pattern")]
    CommitterEmailPattern,
}

impl Default for Type {
    fn default() -> Type {
        Self::CommitterEmailPattern
    }
}

