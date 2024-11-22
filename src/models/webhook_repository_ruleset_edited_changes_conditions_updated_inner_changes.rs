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
pub struct WebhookRepositoryRulesetEditedChangesConditionsUpdatedInnerChanges {
    #[serde(rename = "condition_type", skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<Box<models::WebhookOrganizationRenamedChangesLogin>>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<Box<models::WebhookOrganizationRenamedChangesLogin>>,
    #[serde(rename = "include", skip_serializing_if = "Option::is_none")]
    pub include: Option<Box<models::WebhookRepositoryRulesetEditedChangesConditionsUpdatedInnerChangesInclude>>,
    #[serde(rename = "exclude", skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Box<models::WebhookRepositoryRulesetEditedChangesConditionsUpdatedInnerChangesInclude>>,
}

impl WebhookRepositoryRulesetEditedChangesConditionsUpdatedInnerChanges {
    pub fn new() -> WebhookRepositoryRulesetEditedChangesConditionsUpdatedInnerChanges {
        WebhookRepositoryRulesetEditedChangesConditionsUpdatedInnerChanges {
            condition_type: None,
            target: None,
            include: None,
            exclude: None,
        }
    }
}
