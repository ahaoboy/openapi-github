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
pub struct RepositoryRulesetConditions {
    #[serde(rename = "ref_name", skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<Box<models::RepositoryRulesetConditionsRefName>>,
    #[serde(rename = "repository_name")]
    pub repository_name: Box<models::RepositoryRulesetConditionsRepositoryNameTargetRepositoryName>,
    #[serde(rename = "repository_id")]
    pub repository_id: Box<models::RepositoryRulesetConditionsRepositoryIdTargetRepositoryId>,
    #[serde(rename = "repository_property")]
    pub repository_property: Box<models::RepositoryRulesetConditionsRepositoryPropertyTargetRepositoryProperty>,
}

impl RepositoryRulesetConditions {
    pub fn new(repository_name: models::RepositoryRulesetConditionsRepositoryNameTargetRepositoryName, repository_id: models::RepositoryRulesetConditionsRepositoryIdTargetRepositoryId, repository_property: models::RepositoryRulesetConditionsRepositoryPropertyTargetRepositoryProperty) -> RepositoryRulesetConditions {
        RepositoryRulesetConditions {
            ref_name: None,
            repository_name: Box::new(repository_name),
            repository_id: Box::new(repository_id),
            repository_property: Box::new(repository_property),
        }
    }
}
