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

/// WebhookTeamEditedChanges : The changes to the team if the action was `edited`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookTeamEditedChanges {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<models::WebhookLabelEditedChangesDescription>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<models::WebhookLabelEditedChangesName>>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Box<models::WebhookTeamEditedChangesPrivacy>>,
    #[serde(rename = "notification_setting", skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<Box<models::WebhookTeamEditedChangesNotificationSetting>>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::WebhookTeamEditedChangesRepository>>,
}

impl WebhookTeamEditedChanges {
    /// The changes to the team if the action was `edited`.
    pub fn new() -> WebhookTeamEditedChanges {
        WebhookTeamEditedChanges {
            description: None,
            name: None,
            privacy: None,
            notification_setting: None,
            repository: None,
        }
    }
}
